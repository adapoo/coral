use anyhow::Result;
use serenity::all::{
    ButtonStyle, ComponentInteraction, ComponentInteractionDataKind, Context, CreateActionRow,
    CreateButton, CreateComponent, CreateContainer, CreateContainerComponent, CreateInputText,
    CreateInteractionResponse, CreateModal, CreateModalComponent, CreateSection,
    CreateSectionAccessory, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
    InputTextStyle, ModalInteraction,
};

use database::{AccountRepository, MemberRepository};

use super::manage::fetch_context;
use crate::commands::blacklist::channel;
use crate::framework::{AccessRank, Data};
use crate::interact;
use crate::utils::{resolve_username, separator, text};

fn context_prefix(custom_id: &str) -> &'static str {
    if custom_id.starts_with("dashboard_") {
        "dashboard"
    } else {
        "manage"
    }
}

pub async fn build_accounts_panel(
    data: &Data,
    can_modify: bool,
    target_id: u64,
    member: &database::Member,
    prefix: &str,
) -> CreateComponent<'static> {
    let accounts = AccountRepository::new(data.db.pool());
    let alts = accounts.list(member.id).await.unwrap_or_default();

    let mut parts: Vec<CreateContainerComponent> = vec![text("### Accounts")];

    if let Some(uuid) = &member.uuid {
        let username = resolve_username(uuid, data).await;
        let name = username.as_deref().unwrap_or(uuid);

        parts.push(separator());
        parts.push(text("**Primary**"));

        if !alts.is_empty() {
            let mut options = vec![
                CreateSelectMenuOption::new(name.to_string(), uuid.to_string())
                    .default_selection(true),
            ];

            for alt in &alts {
                let alt_name = resolve_username(&alt.uuid, data).await;
                let label = alt_name.unwrap_or_else(|| alt.uuid.clone());
                options.push(CreateSelectMenuOption::new(label, alt.uuid.clone()));
            }

            let select = CreateSelectMenu::new(
                format!("{prefix}_swap_primary:{target_id}"),
                CreateSelectMenuKind::String {
                    options: options.into(),
                },
            )
            .disabled(!can_modify);

            parts.push(CreateContainerComponent::ActionRow(
                CreateActionRow::SelectMenu(select),
            ));
        } else {
            parts.push(text(format!("**`{name}`**")));
        }

        parts.push(text(format!("-# UUID: {uuid}")));

        if alts.is_empty() {
            parts.push(CreateContainerComponent::ActionRow(
                CreateActionRow::buttons(vec![
                    CreateButton::new(format!("{prefix}_remove_account:{target_id}:{uuid}"))
                        .label("Remove")
                        .style(ButtonStyle::Danger)
                        .disabled(!can_modify),
                ]),
            ));
        }
    }

    for alt in &alts {
        let alt_name = resolve_username(&alt.uuid, data).await;
        let name = alt_name.as_deref().unwrap_or("Unknown");
        let remove = CreateButton::new(format!("{prefix}_remove_account:{target_id}:{}", alt.uuid))
            .label("Remove")
            .style(ButtonStyle::Danger)
            .disabled(!can_modify);
        parts.push(separator());
        parts.push(CreateContainerComponent::Section(CreateSection::new(
            vec![interact::section_text(&format!(
                "**`{name}`**\n-# UUID: {}",
                alt.uuid
            ))],
            CreateSectionAccessory::Button(remove),
        )));
    }

    parts.push(separator());
    parts.push(CreateContainerComponent::ActionRow(
        CreateActionRow::buttons(vec![
            CreateButton::new(format!("{prefix}_add_account:{target_id}"))
                .label("Add Account")
                .style(ButtonStyle::Secondary)
                .disabled(!can_modify),
            CreateButton::new(format!("{prefix}_accounts_back:{target_id}"))
                .label("Back")
                .style(ButtonStyle::Secondary),
        ]),
    ));

    CreateComponent::Container(CreateContainer::new(parts))
}

async fn build_accounts_view(
    data: &Data,
    invoker_rank: AccessRank,
    target_id: u64,
    prefix: &str,
) -> Result<Vec<CreateComponent<'static>>> {
    let repo = MemberRepository::new(data.db.pool());
    let member = repo
        .get_by_discord_id(target_id as i64)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Member not found"))?;
    let target_rank = AccessRank::of(data, target_id, Some(&member));
    let can_modify = prefix == "dashboard" || invoker_rank > target_rank;

    let mut components = if prefix == "dashboard" {
        crate::commands::user::dashboard::build_dashboard_view(&member, data).await
    } else {
        super::manage::build_main_view(data, invoker_rank, target_id).await
    };

    components.push(build_accounts_panel(data, can_modify, target_id, &member, prefix).await);
    Ok(components)
}

async fn refresh_accounts(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
    invoker_rank: AccessRank,
    target_id: u64,
) -> Result<()> {
    let prefix = context_prefix(&component.data.custom_id);
    let components = build_accounts_view(data, invoker_rank, target_id, prefix).await?;
    interact::update_message(ctx, component, components).await
}

async fn refresh_accounts_from_modal(
    ctx: &Context,
    modal: &ModalInteraction,
    data: &Data,
    invoker_rank: AccessRank,
    target_id: u64,
) -> Result<()> {
    let prefix = context_prefix(&modal.data.custom_id);
    let components = build_accounts_view(data, invoker_rank, target_id, prefix).await?;
    interact::update_modal(ctx, modal, components).await
}

pub async fn handle_accounts_button(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let target_id = interact::parse_id(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;

    let invoker_id = component.user.id.get();
    let is_self = invoker_id == target_id;
    let (invoker_rank, _, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if !is_self && (invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank) {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    refresh_accounts(ctx, component, data, invoker_rank, target_id).await
}

pub async fn handle_dashboard_accounts_button(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let target_id = component.user.id.get();
    let invoker_rank = {
        let repo = MemberRepository::new(data.db.pool());
        let member = repo
            .get_by_discord_id(target_id as i64)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Not registered"))?;
        AccessRank::of(data, target_id, Some(&member))
    };

    let components = build_accounts_view(data, invoker_rank, target_id, "dashboard").await?;
    interact::update_message(ctx, component, components).await
}

pub async fn handle_manage_accounts_back(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let target_id = interact::parse_id(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;

    let invoker_id = component.user.id.get();
    let (invoker_rank, _, _) = fetch_context(data, invoker_id, target_id).await?;

    let components = super::manage::build_main_view(data, invoker_rank, target_id).await;
    interact::update_message(ctx, component, components).await
}

pub async fn handle_dashboard_accounts_back(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let discord_id = component.user.id.get() as i64;
    let repo = MemberRepository::new(data.db.pool());

    let Some(member) = repo.get_by_discord_id(discord_id).await? else {
        return interact::send_component_error(ctx, component, "Error", "You are not registered.")
            .await;
    };

    let components = crate::commands::user::dashboard::build_dashboard_view(&member, data).await;
    interact::update_message(ctx, component, components).await
}

pub async fn handle_add_account_button(
    ctx: &Context,
    component: &ComponentInteraction,
) -> Result<()> {
    let target_id = interact::parse_id(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;
    let prefix = context_prefix(&component.data.custom_id);

    let input = CreateInputText::new(InputTextStyle::Short, "username")
        .placeholder("Minecraft username")
        .min_length(1)
        .max_length(16);

    let label = serenity::all::CreateLabel::input_text("Minecraft Username", input);
    let modal = CreateModal::new(
        format!("{prefix}_add_account_modal:{target_id}"),
        "Add Account",
    )
    .components(vec![CreateModalComponent::Label(label)]);

    component
        .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}

pub async fn handle_add_account_modal(
    ctx: &Context,
    modal: &ModalInteraction,
    data: &Data,
) -> Result<()> {
    let prefix = context_prefix(&modal.data.custom_id);
    let target_id = interact::parse_id(&modal.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid modal ID"))?;

    let username = interact::extract_modal_value(&modal.data.components, "username");

    let invoker_id = modal.user.id.get();
    let (invoker_rank, target, target_rank) = fetch_context(data, invoker_id, target_id).await?;
    let is_self = invoker_id == target_id;

    if !is_self && (invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank) {
        return interact::send_modal_error(ctx, modal, "Error", "Insufficient permissions").await;
    }

    let Some(member) = &target else {
        return interact::send_modal_error(ctx, modal, "Error", "User is not registered").await;
    };

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
        crate::accounts::link_alt(ctx, data, target_id, member.id, &uuid).await?;
        return refresh_accounts_from_modal(ctx, modal, data, invoker_rank, target_id).await;
    }

    if is_self && invoker_rank < AccessRank::Moderator {
        return interact::send_modal_error(
            ctx,
            modal,
            "Error",
            "Your Discord must be linked in Hypixel social settings for this account",
        )
        .await;
    }

    let mut components = build_accounts_view(data, invoker_rank, target_id, prefix).await?;

    components.push(CreateComponent::Container(
        CreateContainer::new(vec![
            text(format!(
                "**{}** does not have <@{target_id}>'s Discord linked in Hypixel social settings.",
                stats.username
            )),
            separator(),
            CreateContainerComponent::ActionRow(CreateActionRow::buttons(vec![
                CreateButton::new(format!("{prefix}_force_add:{target_id}:{uuid}"))
                    .label("Force Link")
                    .style(ButtonStyle::Danger),
                CreateButton::new(format!("{prefix}_cancel_add:{target_id}"))
                    .label("Cancel")
                    .style(ButtonStyle::Secondary),
            ])),
        ])
        .accent_color(channel::COLOR_ERROR),
    ));

    interact::update_modal(ctx, modal, components).await
}

pub async fn handle_force_add(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let (target_id, uuid) = interact::parse_ids(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;

    let invoker_id = component.user.id.get();
    let is_self = invoker_id == target_id;
    let (invoker_rank, target, _) = fetch_context(data, invoker_id, target_id).await?;

    if !is_self && invoker_rank < AccessRank::Moderator {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    let Some(member) = &target else {
        return interact::send_component_error(ctx, component, "Error", "User is not registered")
            .await;
    };

    crate::accounts::link_alt(ctx, data, target_id, member.id, &uuid).await?;

    let prefix = context_prefix(&component.data.custom_id);
    let components = build_accounts_view(data, invoker_rank, target_id, prefix).await?;
    interact::update_message(ctx, component, components).await
}

pub async fn handle_remove_account(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let (target_id, uuid) = interact::parse_ids(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;

    let invoker_id = component.user.id.get();
    let is_self = invoker_id == target_id;
    let (invoker_rank, target, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if !is_self && (invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank) {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    let Some(member) = &target else {
        return interact::send_component_error(ctx, component, "Error", "User is not registered")
            .await;
    };

    if member.uuid.as_deref() == Some(&uuid) {
        let repo = MemberRepository::new(data.db.pool());
        repo.clear_uuid(target_id as i64).await?;
    } else {
        let accounts = AccountRepository::new(data.db.pool());
        accounts.remove(member.id, &uuid).await?;
    }

    refresh_accounts(ctx, component, data, invoker_rank, target_id).await
}

pub async fn handle_swap_primary(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let target_id = interact::parse_id(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid select ID"))?;

    let new_uuid = match &component.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => values.first().map(|s| s.as_str()),
        _ => None,
    };

    let Some(new_uuid) = new_uuid else {
        return Ok(());
    };

    let invoker_id = component.user.id.get();
    let is_self = invoker_id == target_id;
    let (invoker_rank, target, target_rank) = fetch_context(data, invoker_id, target_id).await?;

    if !is_self && (invoker_rank < AccessRank::Moderator || invoker_rank <= target_rank) {
        return interact::send_component_error(ctx, component, "Error", "Insufficient permissions")
            .await;
    }

    let Some(member) = &target else {
        return interact::send_component_error(ctx, component, "Error", "User is not registered")
            .await;
    };

    let old_primary = member.uuid.as_deref().unwrap_or("");

    if new_uuid == old_primary {
        return refresh_accounts(ctx, component, data, invoker_rank, target_id).await;
    }

    let accounts = AccountRepository::new(data.db.pool());
    let repo = MemberRepository::new(data.db.pool());

    if !old_primary.is_empty() {
        accounts.add(member.id, old_primary).await?;
    }
    accounts.remove(member.id, new_uuid).await?;
    repo.set_uuid(target_id as i64, new_uuid).await?;

    let user_id = serenity::all::UserId::new(target_id);
    tokio::spawn(crate::sync::sync_user(ctx.clone(), data.clone(), user_id));

    refresh_accounts(ctx, component, data, invoker_rank, target_id).await
}

pub async fn handle_cancel_add(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let target_id = interact::parse_id(&component.data.custom_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid button ID"))?;

    let invoker_id = component.user.id.get();
    let (invoker_rank, _, _) = fetch_context(data, invoker_id, target_id).await?;

    refresh_accounts(ctx, component, data, invoker_rank, target_id).await
}
