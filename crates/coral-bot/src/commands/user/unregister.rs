use anyhow::Result;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateComponent, CreateContainer,
    CreateContainerComponent, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateTextDisplay, MessageFlags,
};

use database::MemberRepository;

use crate::framework::Data;

const COLOR_ERROR: u32 = 0xED4245;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("unregister").description("Unlink your Minecraft account from Discord")
}

pub async fn run(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    let discord_id = command.user.id.get() as i64;
    let repo = MemberRepository::new(data.db.pool());

    let member = repo.get_by_discord_id(discord_id).await?;

    let Some(member) = member else {
        return send_error(
            ctx,
            command,
            "## Not Registered\nYou don't have a linked Minecraft account.",
        )
        .await;
    };

    let Some(uuid) = &member.uuid else {
        return send_error(
            ctx,
            command,
            "## Not Linked\nYou don't have a Minecraft account linked.",
        )
        .await;
    };

    let username = data
        .api
        .resolve(uuid)
        .await
        .map(|r| r.username)
        .unwrap_or_else(|_| uuid.clone());

    repo.clear_uuid(discord_id).await?;

    let container = CreateComponent::Container(
        CreateContainer::new(vec![CreateContainerComponent::TextDisplay(
            CreateTextDisplay::new(format!(
                "## Account Unlinked\n**{username}** has been unlinked.\n\nUse `/register` to link a new account."
            )),
        )]),
    );

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL)
                    .components(vec![container]),
            ),
        )
        .await?;

    Ok(())
}

async fn send_error(
    ctx: &Context,
    command: &CommandInteraction,
    text: &str,
) -> Result<()> {
    let container = CreateComponent::Container(
        CreateContainer::new(vec![CreateContainerComponent::TextDisplay(
            CreateTextDisplay::new(text.to_string()),
        )])
        .accent_color(COLOR_ERROR),
    );

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL)
                    .components(vec![container]),
            ),
        )
        .await?;

    Ok(())
}
