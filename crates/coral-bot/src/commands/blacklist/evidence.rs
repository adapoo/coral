use anyhow::Result;
use blacklist::lookup as lookup_tag;
use database::BlacklistRepository;
use serenity::all::{
    ButtonStyle, CommandInteraction, CommandOptionType, Component, ComponentInteraction,
    ComponentInteractionDataKind, Context, CreateActionRow, CreateAttachment, CreateButton,
    CreateCommand, CreateCommandOption, CreateComponent, CreateContainer, CreateContainerComponent,
    CreateForumPost, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateMediaGallery, CreateMediaGalleryItem, CreateMessage, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption, CreateTextDisplay, CreateUnfurledMediaItem,
    EditAttachments, EditInteractionResponse, EditMessage, EditThread, Message, MessageFlags,
    MessageId, ResolvedValue, ThreadId,
};

use super::channel::COLOR_DANGER;
use super::tag::get_rank;
use crate::framework::{AccessRank, Data};
use crate::utils::{format_uuid_dashed, separator, text};
use coral_redis::BlacklistEvent;

const QUALIFYING_TAGS: &[&str] = &["closet_cheater", "blatant_cheater", "confirmed_cheater"];

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("confirm")
        .description("Create an evidence post and confirm a cheater tag")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "player", "Player name or UUID")
                .required(true),
        )
}

pub async fn run(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    command.defer_ephemeral(&ctx.http).await?;

    let discord_id = command.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    if rank < AccessRank::Helper {
        return crate::interact::send_deferred_error(
            ctx,
            command,
            "Error",
            "Only helpers and above can use this command",
        )
        .await;
    }

    let Some(forum_id) = data.evidence_forum_id else {
        return crate::interact::send_deferred_error(
            ctx,
            command,
            "Error",
            "Evidence forum channel not configured",
        )
        .await;
    };

    let player_name = command
        .data
        .options()
        .iter()
        .find(|o| o.name == "player")
        .and_then(|o| match o.value {
            ResolvedValue::String(s) => Some(s),
            _ => None,
        })
        .unwrap_or("");

    let player_info = match data.api.resolve(player_name).await {
        Ok(info) => info,
        Err(_) => {
            return crate::interact::send_deferred_error(ctx, command, "Error", "Player not found")
                .await;
        }
    };

    let repo = BlacklistRepository::new(data.db.pool());
    let tags = repo.get_tags(&player_info.uuid).await?;

    let qualifying_tag = tags
        .iter()
        .find(|t| QUALIFYING_TAGS.contains(&t.tag_type.as_str()));

    let Some(tag) = qualifying_tag else {
        return crate::interact::send_deferred_error(
            ctx,
            command,
            "Error",
            "Player does not have a closet cheater, blatant cheater, or confirmed cheater tag",
        )
        .await;
    };

    let original_type = tag.tag_type.clone();

    let thread_title = format!(
        "{} | {}",
        player_info.username,
        format_uuid_dashed(&player_info.uuid)
    );
    let message_content = build_evidence_message(
        &player_info.username,
        &player_info.uuid,
        &original_type,
        &[],
        None,
    );

    let forum_post = CreateForumPost::new(
        thread_title,
        CreateMessage::new()
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(message_content),
    );

    let thread = forum_id.create_forum_post(&ctx.http, forum_post).await?;
    let thread_url = format!(
        "https://discord.com/channels/{}/{}",
        command.guild_id.map(|g| g.get()).unwrap_or(0),
        thread.id.get(),
    );

    repo.set_evidence_thread(&player_info.uuid, &thread_url)
        .await?;

    let confirmed_def = lookup_tag("confirmed_cheater");
    let emote = confirmed_def.map(|d| d.emote).unwrap_or("");

    command
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new()
                .flags(MessageFlags::IS_COMPONENTS_V2)
                .components(vec![CreateComponent::Container(CreateContainer::new(
                    vec![CreateContainerComponent::TextDisplay(
                        CreateTextDisplay::new(format!(
                            "## {} Evidence Post Created\nPlayer: `{}`\nThread: <#{}>",
                            emote,
                            player_info.username,
                            thread.id.get()
                        )),
                    )],
                ))]),
        )
        .await?;

    Ok(())
}

#[derive(Debug, Clone)]
struct EvidenceItem {
    filename: String,
}

struct EvidenceState {
    username: String,
    uuid: String,
    original_type: String,
    evidence: Vec<EvidenceItem>,
    review_url: Option<String>,
}

fn url_extension(url: &str) -> &str {
    url.rsplit('/')
        .next()
        .unwrap_or("png")
        .split('?')
        .next()
        .unwrap_or("png")
        .rsplit('.')
        .next()
        .unwrap_or("png")
}

fn build_evidence_message(
    username: &str,
    uuid: &str,
    original_type: &str,
    evidence: &[EvidenceItem],
    review_thread_url: Option<&str>,
) -> Vec<CreateComponent<'static>> {
    let confirmed_def = lookup_tag("confirmed_cheater");
    let emote = confirmed_def.map(|d| d.emote).unwrap_or("");
    let original_def = lookup_tag(original_type);
    let original_display = original_def
        .map(|d| d.display_name)
        .unwrap_or(original_type);

    let dashed_uuid = format_uuid_dashed(uuid);

    let mut header = format!(
        "## {emote} Evidence — `{username}`\nUUID: `{dashed_uuid}`\nTag: Confirmed Cheater (was: {original_display})\n-# Originally: {original_type}"
    );

    if let Some(url) = review_thread_url {
        header.push_str(&format!("\nReview: {url}"));
    }

    let mut parts: Vec<CreateContainerComponent<'static>> =
        vec![text(header), separator(), text("**Evidence**")];

    if evidence.is_empty() {
        parts.push(text("-# No evidence added yet"));
    } else {
        let gallery_items: Vec<CreateMediaGalleryItem<'static>> = evidence
            .iter()
            .map(|e| {
                let url = format!("attachment://{}", e.filename);
                CreateMediaGalleryItem::new(CreateUnfurledMediaItem::new(url))
            })
            .collect();

        parts.push(CreateContainerComponent::MediaGallery(
            CreateMediaGallery::new(gallery_items),
        ));
    }

    parts.push(separator());

    if !evidence.is_empty() {
        let options: Vec<CreateSelectMenuOption<'static>> = evidence
            .iter()
            .enumerate()
            .map(|(i, e)| CreateSelectMenuOption::new(e.filename.clone(), i.to_string()))
            .collect();

        parts.push(CreateContainerComponent::ActionRow(
            CreateActionRow::SelectMenu(
                CreateSelectMenu::new(
                    "evidence_remove",
                    CreateSelectMenuKind::String {
                        options: options.into(),
                    },
                )
                .placeholder("Remove evidence..."),
            ),
        ));
    }

    parts.push(CreateContainerComponent::ActionRow(
        CreateActionRow::Buttons(
            vec![
                CreateButton::new("evidence_add_media")
                    .label("Add Media")
                    .style(ButtonStyle::Secondary),
                CreateButton::new("evidence_archive")
                    .label("Archive")
                    .style(ButtonStyle::Danger),
            ]
            .into(),
        ),
    ));

    let container = CreateContainer::new(parts);
    vec![CreateComponent::Container(container)]
}

fn build_archived_evidence_message(
    state: &EvidenceState,
    reverted_display: &str,
) -> Vec<CreateComponent<'static>> {
    let confirmed_def = lookup_tag("confirmed_cheater");
    let emote = confirmed_def.map(|d| d.emote).unwrap_or("");
    let dashed_uuid = format_uuid_dashed(&state.uuid);

    let mut header = format!(
        "## {emote} Evidence — `{username}` (Archived)\nUUID: `{dashed_uuid}`\nTag: Reverted to {reverted_display} (was: Confirmed Cheater)\n-# Originally: {original_type}",
        username = state.username,
        original_type = state.original_type,
    );

    if let Some(url) = &state.review_url {
        header.push_str(&format!("\nReview: {url}"));
    }

    let mut parts: Vec<CreateContainerComponent<'static>> =
        vec![text(header), separator(), text("**Evidence (Archived)**")];

    if !state.evidence.is_empty() {
        let gallery_items: Vec<CreateMediaGalleryItem<'static>> = state
            .evidence
            .iter()
            .map(|e| {
                let url = format!("attachment://{}", e.filename);
                CreateMediaGalleryItem::new(CreateUnfurledMediaItem::new(url))
            })
            .collect();

        parts.push(CreateContainerComponent::MediaGallery(
            CreateMediaGallery::new(gallery_items),
        ));
    }

    parts.push(separator());

    let container = CreateContainer::new(parts).accent_color(COLOR_DANGER);
    vec![CreateComponent::Container(container)]
}

fn parse_state_from_message(message: &Message) -> Option<EvidenceState> {
    let container = message.components.iter().find_map(|c| match c {
        Component::Container(c) => Some(c),
        _ => None,
    })?;

    let mut username = String::new();
    let mut uuid = String::new();
    let mut original_type = String::new();
    let mut evidence = Vec::new();
    let mut review_url = None;

    for part in &container.components {
        match part {
            serenity::all::ContainerComponent::TextDisplay(td) => {
                let content = td.content.as_deref().unwrap_or("");

                for line in content.lines() {
                    if line.starts_with("UUID: `") {
                        uuid = line
                            .trim_start_matches("UUID: `")
                            .trim_end_matches('`')
                            .replace('-', "");
                    }

                    if let Some(name_part) = line.strip_prefix("## ") {
                        if let Some(after_dash) = name_part.split(" — `").nth(1) {
                            username = after_dash
                                .trim_end_matches('`')
                                .trim_end_matches(" (Archived)")
                                .to_string();
                        }
                    }

                    if let Some(rest) = line.strip_prefix("-# Originally: ") {
                        original_type = rest.trim().to_string();
                    }

                    if let Some(rest) = line.strip_prefix("Review: ") {
                        review_url = Some(rest.trim().to_string());
                    }
                }
            }
            serenity::all::ContainerComponent::MediaGallery(gallery) => {
                for item in &*gallery.items {
                    let url = item
                        .media
                        .proxy_url
                        .as_ref()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| item.media.url.to_string());

                    if !url.is_empty() {
                        let filename = url.rsplit('/').next().unwrap_or("evidence.png");
                        let filename = filename.split('?').next().unwrap_or(filename);
                        evidence.push(EvidenceItem {
                            filename: filename.to_string(),
                        });
                    }
                }
            }
            _ => {}
        }
    }

    if uuid.is_empty() {
        return None;
    }

    Some(EvidenceState {
        username,
        uuid,
        original_type,
        evidence,
        review_url,
    })
}

fn find_upload_prompt(message: &Message) -> bool {
    for component in &message.components {
        let Component::Container(container) = component else {
            continue;
        };
        for part in &container.components {
            let serenity::all::ContainerComponent::ActionRow(row) = part else {
                continue;
            };
            for item in &row.components {
                if let serenity::all::ActionRowComponent::Button(btn) = item {
                    if let serenity::all::ButtonKind::NonLink { custom_id, .. } = &btn.data {
                        if custom_id.as_str() == "evidence_cancel_upload" {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

pub async fn handle_add_media(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    let state = parse_state_from_message(&*component.message);
    let username = state
        .as_ref()
        .map(|s| s.username.clone())
        .unwrap_or_default();

    let container = CreateContainer::new(vec![
        text(format!(
            "Upload media evidence for **`{username}`** in this thread."
        )),
        CreateContainerComponent::ActionRow(CreateActionRow::Buttons(
            vec![
                CreateButton::new("evidence_cancel_upload")
                    .label("Cancel")
                    .style(ButtonStyle::Secondary),
            ]
            .into(),
        )),
    ]);

    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(vec![CreateComponent::Container(container)]),
            ),
        )
        .await?;

    Ok(())
}

pub async fn handle_cancel_upload(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    let message = component.message.clone();
    component
        .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
        .await?;
    let _ = message.delete(&ctx.http, None).await;
    Ok(())
}

pub async fn handle_remove(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let discord_id = component.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    if rank < AccessRank::Helper {
        return crate::interact::send_component_error(
            ctx,
            component,
            "Error",
            "Only helpers and above can remove evidence",
        )
        .await;
    }

    let idx: usize = match &component.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => {
            values.first().and_then(|v| v.parse().ok()).unwrap_or(0)
        }
        _ => return Ok(()),
    };

    let channel_id = component.channel_id;
    let builder_msg_id = MessageId::new(channel_id.get());
    let Ok(builder_msg) = ctx
        .http
        .get_message(channel_id.into(), builder_msg_id)
        .await
    else {
        return Ok(());
    };

    let Some(mut state) = parse_state_from_message(&builder_msg) else {
        return crate::interact::send_component_error(
            ctx,
            component,
            "Error",
            "Could not parse evidence state",
        )
        .await;
    };

    if idx >= state.evidence.len() {
        return crate::interact::send_component_error(
            ctx,
            component,
            "Error",
            "Invalid evidence index",
        )
        .await;
    }

    let removed_filename = state.evidence[idx].filename.clone();
    state.evidence.remove(idx);

    let components = build_evidence_message(
        &state.username,
        &state.uuid,
        &state.original_type,
        &state.evidence,
        state.review_url.as_deref(),
    );

    let mut edit = EditMessage::new()
        .content("")
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(components);

    let mut attachments = EditAttachments::keep_all(&builder_msg);
    if let Some(att) = builder_msg
        .attachments
        .iter()
        .find(|a| a.filename == removed_filename)
    {
        attachments = attachments.remove(att.id);
    }
    edit = edit.attachments(attachments);

    component
        .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
        .await?;

    ctx.http
        .edit_message(channel_id.into(), builder_msg_id, &edit, Vec::new())
        .await?;

    Ok(())
}

pub async fn archive_evidence_by_url(ctx: &Context, data: &Data, thread_url: &str) -> Result<()> {
    let Some(id_str) = thread_url.rsplit('/').next() else {
        return Ok(());
    };
    let Ok(id) = id_str.parse::<u64>() else {
        return Ok(());
    };

    let thread_id = ThreadId::new(id);
    let channel_id: serenity::all::GenericChannelId = thread_id.into();
    let builder_msg_id = MessageId::new(id);

    let Ok(builder_msg) = ctx.http.get_message(channel_id, builder_msg_id).await else {
        return Ok(());
    };

    let Some(state) = parse_state_from_message(&builder_msg) else {
        return Ok(());
    };

    let repo = BlacklistRepository::new(data.db.pool());

    if !state.original_type.is_empty() && state.original_type != "confirmed_cheater" {
        let tags = repo.get_tags(&state.uuid).await?;
        if let Some(confirmed_tag) = tags.iter().find(|t| t.tag_type == "confirmed_cheater") {
            repo.revert_tag_from_confirmed(confirmed_tag.id, &state.original_type)
                .await?;
        }
    }

    repo.clear_evidence_thread(&state.uuid).await?;

    let reverted_display = lookup_tag(&state.original_type)
        .map(|d| d.display_name)
        .unwrap_or(&state.original_type);

    let archived_message = build_archived_evidence_message(&state, reverted_display);

    let edit = EditMessage::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(archived_message)
        .attachments(EditAttachments::keep_all(&builder_msg));

    let _ = ctx
        .http
        .edit_message(channel_id, builder_msg_id, &edit, Vec::new())
        .await;

    let _ = thread_id
        .edit(&ctx.http, EditThread::new().archived(true).locked(true))
        .await;

    Ok(())
}

pub async fn handle_archive(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let discord_id = component.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    if rank < AccessRank::Helper {
        return crate::interact::send_component_error(
            ctx,
            component,
            "Error",
            "Only helpers and above can archive evidence",
        )
        .await;
    }

    let Some(state) = parse_state_from_message(&*component.message) else {
        return crate::interact::send_component_error(
            ctx,
            component,
            "Error",
            "Could not parse evidence state",
        )
        .await;
    };

    let repo = BlacklistRepository::new(data.db.pool());

    if !state.original_type.is_empty() && state.original_type != "confirmed_cheater" {
        let tags = repo.get_tags(&state.uuid).await?;
        if let Some(confirmed_tag) = tags.iter().find(|t| t.tag_type == "confirmed_cheater") {
            repo.revert_tag_from_confirmed(confirmed_tag.id, &state.original_type)
                .await?;
        }
    }

    repo.clear_evidence_thread(&state.uuid).await?;

    let reverted_display = lookup_tag(&state.original_type)
        .map(|d| d.display_name)
        .unwrap_or(&state.original_type);

    let archived_message = build_archived_evidence_message(&state, reverted_display);

    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(archived_message),
            ),
        )
        .await?;

    let thread_id = ThreadId::new(component.channel_id.get());
    let _ = thread_id
        .edit(&ctx.http, EditThread::new().archived(true).locked(true))
        .await;

    Ok(())
}

fn collect_attachment_urls(message: &Message) -> Vec<(String, String)> {
    let direct = message
        .attachments
        .iter()
        .map(|a| (a.url.to_string(), a.filename.to_string()));

    let forwarded = message
        .message_snapshots
        .iter()
        .flat_map(|s| s.attachments.iter())
        .map(|a| (a.url.to_string(), a.filename.to_string()));

    direct.chain(forwarded).collect()
}

pub async fn handle_attachment_message(
    ctx: &Context,
    message: &Message,
    data: &Data,
) -> Result<()> {
    let attachments = collect_attachment_urls(message);
    if attachments.is_empty() {
        return Ok(());
    }

    let Some(evidence_forum_id) = data.evidence_forum_id else {
        return Ok(());
    };

    let channel_id = message.channel_id;
    let channel = ctx.http.get_channel(channel_id.into()).await?;

    let parent_id = channel
        .clone()
        .thread()
        .map(|t| t.parent_id.get())
        .or_else(|| channel.guild().and_then(|c| c.parent_id).map(|id| id.get()));

    if parent_id != Some(evidence_forum_id.get()) {
        return Ok(());
    }

    let builder_msg_id = MessageId::new(channel_id.get());
    let Ok(builder_msg) = ctx
        .http
        .get_message(channel_id.into(), builder_msg_id)
        .await
    else {
        tracing::warn!(
            "Evidence: could not fetch builder message for thread {}",
            channel_id
        );
        return Ok(());
    };

    let Some(mut state) = parse_state_from_message(&builder_msg) else {
        tracing::warn!(
            "Evidence: failed to parse state from builder message (components: {})",
            builder_msg.components.len()
        );
        return Ok(());
    };

    let messages = ctx
        .http
        .get_messages(channel_id.into(), None, None)
        .await
        .unwrap_or_default();

    let prompt_msg = messages
        .iter()
        .find(|m| m.author.bot() && find_upload_prompt(m));
    let prompt_msg_id = prompt_msg.map(|m| m.id);

    let existing_count = state.evidence.len();

    let mut files = Vec::new();
    for (i, (url, orig_filename)) in attachments.iter().enumerate() {
        let ext = url_extension(orig_filename);
        let filename = format!("{}_{}.{}", state.username, existing_count + i + 1, ext);
        let att = CreateAttachment::url(&ctx.http, url.as_str(), filename.clone()).await?;
        files.push(att);
        state.evidence.push(EvidenceItem {
            filename: filename.clone(),
        });
    }

    let components = build_evidence_message(
        &state.username,
        &state.uuid,
        &state.original_type,
        &state.evidence,
        state.review_url.as_deref(),
    );

    let mut edit = EditMessage::new()
        .content("")
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(components);

    let mut attachments = EditAttachments::keep_all(&builder_msg);
    for f in &files {
        attachments = attachments.add(f.clone());
    }
    edit = edit.attachments(attachments);

    ctx.http
        .edit_message(channel_id.into(), builder_msg.id, &edit, files)
        .await?;

    if existing_count == 0
        && !state.original_type.is_empty()
        && state.original_type != "confirmed_cheater"
    {
        let repo = BlacklistRepository::new(data.db.pool());
        let tags = repo.get_tags(&state.uuid).await?;
        if let Some(tag) = tags.iter().find(|t| t.tag_type == state.original_type) {
            let old_tag_type = tag.tag_type.clone();
            let old_reason = tag.reason.clone();
            let old_tag_id = tag.id;
            repo.convert_tag_to_confirmed(tag.id).await?;
            if let Some(updated_tag) = repo.get_tag_by_id(tag.id).await? {
                let event = BlacklistEvent::TagOverwritten {
                    uuid: state.uuid.clone(),
                    old_tag_id,
                    old_tag_type,
                    old_reason,
                    new_tag_id: updated_tag.id,
                    overwritten_by: message.author.id.get() as i64,
                };
                data.event_publisher.publish(&event).await;
            }
        }
    }

    let _ = message.delete(&ctx.http, None).await;
    if let Some(prompt_id) = prompt_msg_id {
        let _ = ctx
            .http
            .delete_message(channel_id.into(), prompt_id, None)
            .await;
    }

    Ok(())
}

pub async fn create_evidence_from_review(
    ctx: &Context,
    data: &Data,
    guild_id: u64,
    uuid: &str,
    username: &str,
    original_type: &str,
    tag_id: i64,
    media_urls: &[String],
    review_thread_url: Option<&str>,
    approved_by: i64,
) -> Result<String> {
    let Some(forum_id) = data.evidence_forum_id else {
        anyhow::bail!("Evidence forum channel not configured");
    };

    let repo = BlacklistRepository::new(data.db.pool());

    let already_confirmed = repo
        .get_tag_by_id(tag_id)
        .await?
        .map(|t| t.tag_type == "confirmed_cheater")
        .unwrap_or(false);

    if !already_confirmed {
        repo.convert_tag_to_confirmed(tag_id).await?;
    }

    let mut evidence: Vec<EvidenceItem> = Vec::new();
    let mut files: Vec<CreateAttachment<'static>> = Vec::new();

    for (i, url) in media_urls.iter().enumerate() {
        let ext = url_extension(url);
        let filename = format!("{}_{}.{}", username, i + 1, ext);
        if let Ok(att) = CreateAttachment::url(&ctx.http, url, filename.clone()).await {
            evidence.push(EvidenceItem {
                filename: filename.clone(),
            });
            files.push(att);
        }
    }

    let thread_title = format!("{} | {}", username, format_uuid_dashed(uuid));
    let initial_components =
        build_evidence_message(username, uuid, original_type, &[], review_thread_url);

    let message = CreateMessage::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(initial_components);

    let forum_post = CreateForumPost::new(thread_title, message);
    let thread = forum_id.create_forum_post(&ctx.http, forum_post).await?;

    if !files.is_empty() {
        let builder_msg_id = MessageId::new(thread.id.get());
        let channel_id: serenity::all::GenericChannelId = thread.id.into();

        let mut edit = EditMessage::new()
            .content("")
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(build_evidence_message(
                username,
                uuid,
                original_type,
                &evidence,
                review_thread_url,
            ));

        let mut attachments = EditAttachments::new();
        for f in &files {
            attachments = attachments.add(f.clone());
        }
        edit = edit.attachments(attachments);

        ctx.http
            .edit_message(channel_id, builder_msg_id, &edit, files)
            .await?;
    }

    let thread_url = format!(
        "https://discord.com/channels/{}/{}",
        guild_id,
        thread.id.get()
    );

    repo.set_evidence_thread(uuid, &thread_url).await?;

    if !already_confirmed {
        if let Ok(Some(_tag)) = repo.get_tag_by_id(tag_id).await {
            let event = BlacklistEvent::TagAdded {
                uuid: uuid.to_string(),
                tag_id,
                added_by: approved_by,
            };
            data.event_publisher.publish(&event).await;
        }
    }

    Ok(thread_url)
}
