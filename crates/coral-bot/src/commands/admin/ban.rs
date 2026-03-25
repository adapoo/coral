use anyhow::{Result, anyhow};
use serenity::all::*;

use database::MemberRepository;

use crate::commands::blacklist::channel;
use crate::framework::{AccessRank, AccessRankExt, Data};
use crate::interact;


const COLOR_SUCCESS: u32 = 0x00FF00;


pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("ban")
        .description("Revoke a user's API key and lock their account")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "User to ban").required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "reason", "Reason for ban")
                .required(true),
        )
}


pub async fn run(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    let invoker_id = command.user.id.get();
    let repo = MemberRepository::new(data.db.pool());
    let invoker = repo.get_by_discord_id(invoker_id as i64).await?;
    let rank = AccessRank::of(data, invoker_id, invoker.as_ref());

    if rank < AccessRank::Admin {
        return interact::send_error(
            ctx, command, "Error", "You don't have permission to use this command",
        )
        .await;
    }

    let target_id = extract_user_option(&command.data.options, "user")
        .ok_or_else(|| anyhow!("Missing user"))?;
    let reason = command.data.options.iter()
        .find(|o| o.name == "reason")
        .and_then(|o| o.value.as_str())
        .unwrap_or("No reason provided");

    let revoked = repo.revoke_api_key(target_id as i64).await?;
    if revoked {
        channel::post_key_revoked(ctx, data, target_id, reason, invoker_id).await;
    }

    let embed = match revoked {
        true => CreateEmbed::new()
            .title("User Banned")
            .description(format!("<@{target_id}> has been banned\n\n**Reason:** {reason}"))
            .color(Color::new(COLOR_SUCCESS)),
        false => CreateEmbed::new()
            .title("User Not Found")
            .description(format!("<@{target_id}> is not registered"))
            .color(Color::new(channel::COLOR_ERROR)),
    };

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await?;
    Ok(())
}


fn extract_user_option(options: &[CommandDataOption], name: &str) -> Option<u64> {
    options.iter()
        .find(|o| o.name == name)
        .and_then(|o| match &o.value {
            CommandDataOptionValue::User(id) => Some(id.get()),
            _ => None,
        })
}
