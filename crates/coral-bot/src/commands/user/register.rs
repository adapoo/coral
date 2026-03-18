use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use serenity::all::{
    ButtonStyle, CommandInteraction, CommandOptionType, ComponentInteraction, Context,
    CreateActionRow, CreateButton, CreateCommand, CreateCommandOption, CreateComponent,
    CreateContainer, CreateContainerComponent, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateLabel, CreateModal, CreateModalComponent,
    CreateSeparator, CreateTextDisplay, InputTextStyle, Member, MessageFlags, ModalInteraction,
    RoleId,
};

use database::{GuildConfigRepository, MemberRepository};

use crate::accounts::{self, LinkCheck};
use crate::framework::Data;
use crate::interact;

const RETRY_COOLDOWN: Duration = Duration::from_secs(30);

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("register")
        .description("Link your Minecraft account to Discord")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "account",
                "Your Minecraft username or UUID",
            )
            .required(true),
        )
}

pub async fn run(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    let player = command
        .data
        .options
        .first()
        .and_then(|o| o.value.as_str())
        .unwrap_or("");

    let discord_id = command.user.id.get();

    match accounts::check_link(data, player, &command.user.name).await {
        LinkCheck::Verified { uuid, username, .. } => {
            accounts::link_primary(ctx, data, discord_id, &uuid).await?;
            send_response(ctx, command, build_success_container(&username)).await
        }
        LinkCheck::NotVerified { uuid, .. } => {
            send_response(ctx, command, build_link_instructions_container(&uuid)).await
        }
        LinkCheck::PlayerNotFound => {
            send_response(
                ctx,
                command,
                build_error_container("Player Not Found", "Could not find that Minecraft account."),
            )
            .await
        }
        LinkCheck::HypixelNotFound => {
            send_response(
                ctx,
                command,
                build_error_container("Player Not Found", "Could not find that player on Hypixel."),
            )
            .await
        }
    }
}

pub async fn handle_link_button(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
    let input = CreateInputText::new(InputTextStyle::Short, "username")
        .placeholder("Your Minecraft username")
        .min_length(1)
        .max_length(16);

    let label = CreateLabel::input_text("Minecraft Username", input);
    let modal = CreateModal::new("link_modal", "Link Your Account")
        .components(vec![CreateModalComponent::Label(label)]);

    component
        .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}

pub async fn handle_link_modal(ctx: &Context, modal: &ModalInteraction, data: &Data) -> Result<()> {
    let username = interact::extract_modal_value(&modal.data.components, "username");
    let discord_id = modal.user.id.get();

    match accounts::check_link(data, &username, &modal.user.name).await {
        LinkCheck::Verified { uuid, username, .. } => {
            accounts::link_primary(ctx, data, discord_id, &uuid).await?;
            send_modal_response(ctx, modal, build_success_container(&username)).await
        }
        LinkCheck::NotVerified { uuid, .. } => {
            send_modal_response(ctx, modal, build_link_instructions_container(&uuid)).await
        }
        LinkCheck::PlayerNotFound => {
            send_modal_response(
                ctx,
                modal,
                build_error_container("Player Not Found", "Could not find that Minecraft account."),
            )
            .await
        }
        LinkCheck::HypixelNotFound => {
            send_modal_response(
                ctx,
                modal,
                build_error_container("Player Not Found", "Could not find that player on Hypixel."),
            )
            .await
        }
    }
}

pub async fn handle_retry_button(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let player = component
        .data
        .custom_id
        .strip_prefix("register_retry:")
        .unwrap_or("");

    let user_id = component.user.id;

    let cooldown_remaining = {
        let cooldowns = data.register_cooldowns.lock().unwrap();
        cooldowns.get(&user_id).and_then(|&last| {
            let elapsed = last.elapsed();
            (elapsed < RETRY_COOLDOWN).then(|| RETRY_COOLDOWN - elapsed)
        })
    };

    if let Some(remaining) = cooldown_remaining {
        let expiry = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + remaining.as_secs();

        return send_component_response(
            ctx,
            component,
            build_container(
                &format!("## Cooldown Active\nYou can retry <t:{expiry}:R>."),
                Some(build_retry_row(player)),
            ),
        )
        .await;
    }

    {
        let mut cooldowns = data.register_cooldowns.lock().unwrap();
        cooldowns.insert(user_id, Instant::now());
    }

    let discord_id = user_id.get();

    match accounts::check_link(data, player, &component.user.name).await {
        LinkCheck::Verified { uuid, username, .. } => {
            accounts::link_primary(ctx, data, discord_id, &uuid).await?;
            send_component_response(ctx, component, build_success_container(&username)).await
        }
        LinkCheck::NotVerified { uuid, .. } => {
            send_component_response(ctx, component, build_link_instructions_container(&uuid)).await
        }
        LinkCheck::PlayerNotFound => {
            send_component_response(
                ctx,
                component,
                build_error_container("Player Not Found", "Could not find that Minecraft account."),
            )
            .await
        }
        LinkCheck::HypixelNotFound => {
            send_component_response(
                ctx,
                component,
                build_error_container("Player Not Found", "Could not find that player on Hypixel."),
            )
            .await
        }
    }
}

pub async fn handle_guild_join(ctx: &Context, new_member: &Member, data: &Data) -> Result<()> {
    let discord_id = new_member.user.id.get() as i64;
    let members = MemberRepository::new(data.db.pool());

    let uuid = match members
        .get_by_discord_id(discord_id)
        .await?
        .and_then(|m| m.uuid)
    {
        Some(uuid) => uuid,
        None => {
            assign_unlinked_role(ctx, data, new_member).await;
            return Ok(());
        }
    };

    let stats = match data.api.get_player_stats(&uuid).await {
        Ok(s) => s,
        Err(_) => return Ok(()),
    };

    let Some(hypixel_data) = stats.hypixel else {
        return Ok(());
    };

    let config_repo = GuildConfigRepository::new(data.db.pool());
    let config = match config_repo.get(new_member.guild_id.get() as i64).await {
        Ok(Some(c)) => c,
        _ => return Ok(()),
    };
    let rules = config_repo
        .get_role_rules(new_member.guild_id.get() as i64)
        .await
        .unwrap_or_default();

    if let Err(e) = crate::sync::sync_member(
        ctx,
        data,
        new_member.guild_id,
        new_member,
        &uuid,
        &config,
        &rules,
        &hypixel_data,
        true,
    )
    .await
    {
        tracing::warn!(
            "Failed to sync joining member {} in {}: {e}",
            new_member.user.id,
            new_member.guild_id
        );
    }

    Ok(())
}

async fn assign_unlinked_role(ctx: &Context, data: &Data, member: &Member) {
    let repo = GuildConfigRepository::new(data.db.pool());
    let config = match repo.get(member.guild_id.get() as i64).await {
        Ok(Some(c)) => c,
        _ => return,
    };

    if let Some(role_id) = config.unlinked_role_id {
        let _ = member
            .add_role(&ctx.http, RoleId::new(role_id as u64), None)
            .await;
    }
}

fn build_container(
    text: &str,
    action_row: Option<CreateActionRow<'static>>,
) -> Vec<CreateComponent<'static>> {
    let mut parts: Vec<CreateContainerComponent> = vec![CreateContainerComponent::TextDisplay(
        CreateTextDisplay::new(text.to_string()),
    )];

    if let Some(row) = action_row {
        parts.push(CreateContainerComponent::Separator(CreateSeparator::new(
            true,
        )));
        parts.push(CreateContainerComponent::ActionRow(row));
    }

    vec![CreateComponent::Container(CreateContainer::new(parts))]
}

fn build_retry_row(player: &str) -> CreateActionRow<'static> {
    CreateActionRow::buttons(vec![
        CreateButton::new(format!("register_retry:{player}"))
            .label("Retry")
            .style(ButtonStyle::Primary),
    ])
}

fn build_success_container(ign: &str) -> Vec<CreateComponent<'static>> {
    build_container(
        &format!(
            "## Account Linked\nYour account has been linked to **{ign}**.\n\nUse `/dashboard` to get your API key."
        ),
        None,
    )
}

fn build_error_container(title: &str, description: &str) -> Vec<CreateComponent<'static>> {
    build_container(&format!("## {title}\n{description}"), None)
}

fn build_link_instructions_container(uuid: &str) -> Vec<CreateComponent<'static>> {
    build_container(
        &format!(
            "## Discord Link Required\n\
            To link `{uuid}`, please:\n\n\
            1. Go to Hypixel and open Social Media settings\n\
            2. Set your Discord to your current Discord username\n\
            3. Run this command again\n\n\
            This confirms you own the Minecraft account."
        ),
        Some(build_retry_row(uuid)),
    )
}

async fn send_response(
    ctx: &Context,
    command: &CommandInteraction,
    components: Vec<CreateComponent<'static>>,
) -> Result<()> {
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL)
                    .components(components),
            ),
        )
        .await?;

    Ok(())
}

async fn send_modal_response(
    ctx: &Context,
    modal: &ModalInteraction,
    components: Vec<CreateComponent<'static>>,
) -> Result<()> {
    modal
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL)
                    .components(components),
            ),
        )
        .await?;

    Ok(())
}

async fn send_component_response(
    ctx: &Context,
    component: &ComponentInteraction,
    components: Vec<CreateComponent<'static>>,
) -> Result<()> {
    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL)
                    .components(components),
            ),
        )
        .await?;

    Ok(())
}
