use anyhow::Result;
use serenity::all::{
    Color, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
};

use database::MemberRepository;

use crate::commands::blacklist::channel;
use crate::framework::{AccessRank, Data};

const COLOR_SUCCESS: u32 = 0x00FF00;
const COLOR_ERROR: u32 = 0xFF5555;

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
        return send_error(
            ctx,
            command,
            "You don't have permission to use this command",
        )
        .await;
    }

    let target_id = extract_user_option(&command.data.options, "user")
        .ok_or_else(|| anyhow::anyhow!("Missing user"))?;

    let reason = command
        .data
        .options
        .iter()
        .find(|o| o.name == "reason")
        .and_then(|o| o.value.as_str())
        .unwrap_or("No reason provided");

    let revoked = repo.revoke_api_key(target_id as i64).await?;

    if revoked {
        channel::post_key_revoked(ctx, data, target_id, reason, invoker_id).await;
    }

    let embed = if revoked {
        build_success_embed(target_id, reason)
    } else {
        build_not_found_embed(target_id)
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

fn build_success_embed(user_id: u64, reason: &str) -> CreateEmbed<'static> {
    CreateEmbed::new()
        .title("User Banned")
        .description(format!(
            "<@{}> has been banned\n\n**Reason:** {}",
            user_id, reason
        ))
        .color(Color::new(COLOR_SUCCESS))
}

fn build_not_found_embed(user_id: u64) -> CreateEmbed<'static> {
    CreateEmbed::new()
        .title("User Not Found")
        .description(format!("<@{}> is not registered", user_id))
        .color(Color::new(COLOR_ERROR))
}

async fn send_error(ctx: &Context, command: &CommandInteraction, message: &str) -> Result<()> {
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(message)
                    .ephemeral(true),
            ),
        )
        .await?;
    Ok(())
}

fn extract_user_option(options: &[serenity::all::CommandDataOption], name: &str) -> Option<u64> {
    options
        .iter()
        .find(|o| o.name == name)
        .and_then(|o| match &o.value {
            serenity::all::CommandDataOptionValue::User(id) => Some(id.get()),
            _ => None,
        })
}
