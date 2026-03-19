use anyhow::Result;
use serenity::all::{
    ButtonStyle, CommandInteraction, CommandOptionType, ComponentInteraction,
    ComponentInteractionDataKind, Context, CreateActionRow, CreateButton, CreateCommand,
    CreateCommandOption, CreateComponent, CreateContainer, CreateContainerComponent,
    CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateLabel,
    CreateModal, CreateModalComponent, CreateSection, CreateSectionAccessory, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption, InputTextStyle, MessageFlags, ModalInteraction,
};

use database::MemberRepository;

use crate::commands::blacklist::channel;
use crate::framework::{AccessRank, Data};
use crate::interact;
use crate::utils::{format_number, resolve_username, separator, text};

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("manage")
        .description("Open the user management panel")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "Target user").required(true),
        )
}

pub async fn run(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    let invoker_id = command.user.id.get();
    let repo = MemberRepository::new(data.db.pool());

    let invoker = repo.get_by_discord_id(invoker_id as i64).await?;
    let invoker_rank = AccessRank::of(data, invoker_id, invoker.as_ref());

    if invoker_rank < AccessRank::Moderator {
        return interact::send_error(
            ctx,
            command,
            "Error",
            "You don't have permission to use this command",
        )
        .await;
    }

    let target_id = command
        .data
        .options
        .first()
        .and_then(|o| match &o.value {
            serenity::all::CommandDataOptionValue::User(id) => Some(id.get()),
            _ => None,
        })
        .ok_or_else(|| anyhow::anyhow!("Missing user"))?;

    let components = build_main_view(data, invoker_rank, target_id).await;

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

pub(crate) async fn build_main_view(
    data: &Data,
    invoker_rank: AccessRank,
    target_id: u64,
) -> Vec<CreateComponent<'static>> {
    let repo = MemberRepository::new(data.db.pool());
    let target = repo
        .get_by_discord_id(target_id as i64)
        .await
        .ok()
        .flatten();
    let target_rank = AccessRank::of(data, target_id, target.as_ref());
    let can_modify = invoker_rank > target_rank;

    let mut parts: Vec<CreateContainerComponent> =
        vec![text(format!("## User Management — <@{target_id}>"))];

    match &target {
        Some(m) => {
            parts.push(separator());

            if let Some(uuid) = &m.uuid {
                let username = resolve_username(uuid, data).await;
                let name = username.as_deref().unwrap_or(uuid);
                parts.push(text(format!("**{name}**\n-# UUID: {uuid}")));
            } else {
                parts.push(text("No account linked"));
            }

            parts.push(CreateContainerComponent::ActionRow(
                CreateActionRow::buttons(vec![
                    CreateButton::new(format!("manage_accounts:{target_id}"))
                        .label("Accounts")
                        .style(ButtonStyle::Secondary)
                        .disabled(!can_modify),
                ]),
            ));

            let api_status = if m.key_locked {
                "Locked"
            } else if m.api_key.is_some() {
                "Active"
            } else {
                "None"
            };
            parts.push(text(format!(
                "**API Key** {api_status}\n**Requests** {}",
                format_number(m.request_count as u64)
            )));

            let lock_button = if m.key_locked {
                CreateButton::new(format!("manage_unlock:{target_id}"))
                    .label("Unlock Key")
                    .style(ButtonStyle::Success)
                    .disabled(!can_modify)
            } else {
                CreateButton::new(format!("manage_lock:{target_id}"))
                    .label("Lock Key")
                    .style(ButtonStyle::Danger)
                    .disabled(!can_modify)
            };

            parts.push(CreateContainerComponent::ActionRow(
                CreateActionRow::buttons(vec![lock_button]),
            ));
        }
        None => {
            let register = CreateButton::new(format!("manage_register:{target_id}"))
                .label("Register")
                .style(ButtonStyle::Primary)
                .disabled(!can_modify);
            parts.push(separator());
            parts.push(CreateContainerComponent::Section(CreateSection::new(
                vec![interact::section_text("Not registered")],
                CreateSectionAccessory::Button(register),
            )));
        }
    }

    if target.is_some() {
        parts.push(separator());
        parts.push(text("**Access Level**"));

        let options = access_level_options(invoker_rank, target_rank);
        let disabled = !can_modify || options.is_empty();
        let select = CreateSelectMenu::new(
            format!("manage_access_select:{target_id}"),
            CreateSelectMenuKind::String {
                options: options.into(),
            },
        )
        .placeholder(target_rank.label())
        .disabled(disabled);

        parts.push(CreateContainerComponent::ActionRow(
            CreateActionRow::SelectMenu(select),
        ));
    }

    if let Some(m) = &target {
        if invoker_rank >= AccessRank::Helper && can_modify {
            parts.push(separator());

            let (label, style) = if m.tagging_disabled {
                ("Enable Tagging", ButtonStyle::Success)
            } else {
                ("Disable Tagging", ButtonStyle::Danger)
            };

            parts.push(text("**Tagging**"));
            parts.push(CreateContainerComponent::ActionRow(
                CreateActionRow::buttons(vec![
                    CreateButton::new(format!("manage_toggle_tagging:{target_id}"))
                        .label(label)
                        .style(style),
                ]),
            ));
        }
    }

    if let Some(m) = &target {
        parts.push(separator());
        parts.push(text(format!(
            "**Tag Stats**\nAccepted: {}\nRejected: {}\nAccurate Verdicts: {}",
            m.accepted_tags, m.rejected_tags, m.accurate_verdicts
        )));

        let strikes = m
            .config
            .get("strikes")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if strikes.is_empty() {
            parts.push(text("**Strikes** None"));
        } else {
            let mut strike_text = format!("**Strikes** ({})", strikes.len());
            for (i, strike) in strikes.iter().enumerate() {
                let reason = strike
                    .get("reason")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                let struck_by = strike
                    .get("struck_by")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                strike_text.push_str(&format!("\n{}. \"{}\" — <@{}>", i + 1, reason, struck_by));
            }
            parts.push(text(strike_text));

            if can_modify {
                let mut buttons = Vec::new();
                for i in 0..strikes.len().min(5) {
                    buttons.push(
                        CreateButton::new(format!("manage_remove_strike:{target_id}:{i}"))
                            .label(format!("Remove #{}", i + 1))
                            .style(ButtonStyle::Danger),
                    );
                }
                parts.push(CreateContainerComponent::ActionRow(
                    CreateActionRow::buttons(buttons),
                ));
            }
        }
    }

    vec![CreateComponent::Container(CreateContainer::new(parts))]
}

fn access_level_options(
    invoker_rank: AccessRank,
    current: AccessRank,
) -> Vec<CreateSelectMenuOption<'static>> {
    let levels = [
        (AccessRank::Default, "Default", "Default access"),
        (AccessRank::Member, "Member", "Member access"),
        (AccessRank::Helper, "Helper", "Helper access"),
        (AccessRank::Moderator, "Moderator", "Moderator access"),
        (AccessRank::Admin, "Admin", "Administrator access"),
    ];

    levels
        .into_iter()
        .filter(|(rank, _, _)| *rank < invoker_rank)
        .map(|(rank, label, desc)| {
            CreateSelectMenuOption::new(label, rank.to_level().to_string())
                .description(desc)
                .default_selection(rank == current)
        })
        .collect()
}

pub async fn fetch_context(
    data: &Data,
    invoker_id: u64,
    target_id: u64,
) -> Result<(AccessRank, Option<database::Member>, AccessRank)> {
    let repo = MemberRepository::new(data.db.pool());
    let invoker = repo.get_by_discord_id(invoker_id as i64).await?;
    let invoker_rank = AccessRank::of(data, invoker_id, invoker.as_ref());
    let target = repo.get_by_discord_id(target_id as i64).await?;
    let target_rank = AccessRank::of(data, target_id, target.as_ref());
    Ok((invoker_rank, target, target_rank))
}

async fn refresh_main(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
    invoker_rank: AccessRank,
    target_id: u64,
) -> Result<()> {
    let components = build_main_view(data, invoker_rank, target_id).await;
    interact::update_message(ctx, component, components).await
}

async fn refresh_main_from_modal(
    ctx: &Context,
    modal: &ModalInteraction,
    data: &Data,
    invoker_rank: AccessRank,
    target_id: u64,
) -> Result<()> {
    let components = build_main_view(data, invoker_rank, target_id).await;
    interact::update_modal(ctx, modal, components).await
}

pub async fn handle_access_select(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let value = match &component.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => {
            values.first().and_then(|s| s.parse::<i16>().ok())
        }
        _ => None,
    };

    let Some(new_level) = value else {
        return Ok(());
    };

    let target_id = interact::parse_id(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid select ID"))?;

    let invoker_id = component.user.id.get();
    let (invoker_rank, target, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    let new_rank = AccessRank::from_level(new_level);
    if new_rank >= invoker_rank {
        return interact::send_component_error(
            ctx,
            component,
            "Error",
            "Cannot assign a rank equal to or above your own",
        )
        .await;
    }

    if target.is_none() {
        return interact::send_component_error(ctx, component, "Error", "User is not registered")
            .await;
    }

    let repo = MemberRepository::new(data.db.pool());
    repo.set_access_level(target_id as i64, new_level).await?;

    channel::post_access_changed(ctx, data, target_id, target_rank, new_rank, invoker_id).await;

    refresh_main(ctx, component, data, invoker_rank, target_id).await
}

pub async fn handle_lock_button(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let target_id = interact::parse_id(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;

    let invoker_id = component.user.id.get();
    let (invoker_rank, _, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    let repo = MemberRepository::new(data.db.pool());
    repo.lock_key(target_id as i64).await?;

    channel::post_key_locked(ctx, data, target_id, invoker_id).await;

    refresh_main(ctx, component, data, invoker_rank, target_id).await
}

pub async fn handle_unlock_button(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let target_id = interact::parse_id(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;

    let invoker_id = component.user.id.get();
    let (invoker_rank, _, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    let repo = MemberRepository::new(data.db.pool());
    repo.unlock_key(target_id as i64).await?;

    channel::post_key_unlocked(ctx, data, target_id, invoker_id).await;

    refresh_main(ctx, component, data, invoker_rank, target_id).await
}

pub async fn handle_register_button(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let target_id = interact::parse_id(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;

    let invoker_id = component.user.id.get();
    let (invoker_rank, _, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    let input = CreateInputText::new(InputTextStyle::Short, "username")
        .placeholder("Minecraft username")
        .min_length(1)
        .max_length(16);

    let label = CreateLabel::input_text("Minecraft Username", input);
    let modal = CreateModal::new(
        format!("manage_register_modal:{target_id}"),
        "Register User",
    )
    .components(vec![CreateModalComponent::Label(label)]);

    component
        .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}

pub async fn handle_register_modal(
    ctx: &Context,
    modal: &ModalInteraction,
    data: &Data,
) -> Result<()> {
    let target_id = interact::parse_id(&modal.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid modal ID"))?;

    let username = interact::extract_modal_value(&modal.data.components, "username");

    let invoker_id = modal.user.id.get();
    let (invoker_rank, _, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank {
        return interact::send_modal_error(ctx, modal, "Error", "Insufficient permissions").await;
    }

    let stats = match data.api.get_player_stats(&username).await {
        Ok(s) => s,
        Err(_) => {
            return interact::send_modal_error(
                ctx,
                modal,
                "Error",
                &format!("Could not find player: {username}"),
            )
            .await;
        }
    };

    let uuid = stats.uuid.replace('-', "");
    let discord_user = serenity::all::UserId::new(target_id)
        .to_user(&ctx.http)
        .await;
    let discord_name = discord_user.as_ref().map(|u| u.name.as_str()).unwrap_or("");

    let verified = stats
        .hypixel
        .as_ref()
        .map(|h| crate::accounts::is_discord_linked(h, discord_name))
        .unwrap_or(false);

    if verified {
        crate::accounts::link_primary(ctx, data, target_id, &uuid).await?;
        return refresh_main_from_modal(ctx, modal, data, invoker_rank, target_id).await;
    }

    let container = CreateComponent::Container(
        CreateContainer::new(vec![
            text(format!(
                "## Discord Mismatch\n**{}** (`{uuid}`) does not have <@{target_id}>'s Discord linked in Hypixel social settings.\n\nForce link anyway?",
                stats.username
            )),
            separator(),
            CreateContainerComponent::ActionRow(CreateActionRow::buttons(vec![
                CreateButton::new(format!("manage_force_link:{target_id}:{uuid}"))
                    .label("Force Link")
                    .style(ButtonStyle::Danger),
            ])),
        ])
        .accent_color(channel::COLOR_ERROR),
    );

    modal
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

pub async fn handle_force_link(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let (target_id, uuid) = interact::parse_ids(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;

    let invoker_id = component.user.id.get();
    let (invoker_rank, _, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    crate::accounts::link_primary(ctx, data, target_id, &uuid).await?;

    refresh_main(ctx, component, data, invoker_rank, target_id).await
}

pub async fn handle_toggle_tagging(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let target_id = interact::parse_id(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;

    let invoker_id = component.user.id.get();
    let (invoker_rank, target, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if invoker_rank < AccessRank::Helper || invoker_rank <= target_rank {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    let Some(target_member) = target else {
        return interact::send_component_error(ctx, component, "Error", "User is not registered")
            .await;
    };

    let new_state = !target_member.tagging_disabled;
    let repo = MemberRepository::new(data.db.pool());
    repo.set_tagging_disabled(target_id as i64, new_state)
        .await?;

    channel::post_tagging_toggled(ctx, data, target_id, new_state, invoker_id).await;

    refresh_main(ctx, component, data, invoker_rank, target_id).await
}

pub async fn handle_remove_strike(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let (target_id, strike_index_str) = interact::parse_ids(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;
    let strike_index: usize = strike_index_str
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid strike index"))?;

    let invoker_id = component.user.id.get();
    let (invoker_rank, _, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    let repo = MemberRepository::new(data.db.pool());
    repo.remove_strike(target_id as i64, strike_index).await?;

    refresh_main(ctx, component, data, invoker_rank, target_id).await
}
