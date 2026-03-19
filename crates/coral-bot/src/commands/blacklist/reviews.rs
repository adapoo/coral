use anyhow::Result;
use blacklist::{EMOTE_ADDTAG, EMOTE_TAG, Replay, lookup as lookup_tag, parse_replay};
use database::{BlacklistRepository, MemberRepository};
use serenity::all::{
    ButtonStyle, Component, ComponentInteraction, ComponentInteractionDataKind, Context,
    CreateActionRow, CreateAttachment, CreateButton, CreateComponent, CreateContainer,
    CreateContainerComponent, CreateForumPost, CreateInputText, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateLabel, CreateMediaGallery, CreateMediaGalleryItem,
    CreateMessage, CreateModal, CreateModalComponent, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, CreateUnfurledMediaItem, EditAttachments, EditMessage, EditThread,
    ForumTagId, GenericChannelId, InputTextStyle, Message, MessageFlags, MessageId,
    ModalInteraction, ThreadId,
};

use crate::framework::Data;
use crate::utils::{format_uuid_dashed, sanitize_reason, separator, text};
use coral_redis::BlacklistEvent;

const TAG_PENDING: &str = "Pending";
const TAG_APPROVED: &str = "Approved";
const TAG_REJECTED: &str = "Rejected";
const TAG_NICKED: &str = "Nicked";
const TAG_AWAITING_EVIDENCE: &str = "Awaiting Evidence";

#[derive(Debug, Clone)]
struct PlayerEntry {
    username: String,
    uuid: String,
    tag_type: String,
    reason: String,
    is_nicked: bool,
    status: PlayerStatus,
    reviewer: Option<String>,
    review_note: Option<String>,
    evidence: Vec<Evidence>,
    conflict_warning: Option<String>,
    accept_votes: Vec<u64>,
    reject_votes: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq)]
enum PlayerStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone)]
enum Evidence {
    Replay {
        replay: Replay,
        note: Option<String>,
    },
    Attachment {
        url: String,
    },
}

#[derive(Debug, Clone)]
struct SubmissionState {
    submitter_id: u64,
    players: Vec<PlayerEntry>,
    submitted: bool,
}

const MAX_MEDIA_PER_PLAYER: usize = 4;
const REVIEW_TAGS: &[&str] = &["closet_cheater", "blatant_cheater"];
const SUBMISSION_TIMEOUT_SECS: u64 = 30 * 60;
const SUBMISSION_WARNING_SECS: u64 = 20 * 60;

fn build_tag_select_options(selected: Option<&str>) -> Vec<CreateSelectMenuOption<'static>> {
    blacklist::all()
        .iter()
        .filter(|def| REVIEW_TAGS.contains(&def.name))
        .map(|def| {
            let mut opt = CreateSelectMenuOption::new(def.display_name, def.name);
            if selected == Some(def.name) {
                opt = opt.default_selection(true);
            }
            opt
        })
        .collect()
}

fn extract_modal_value(modal: &ModalInteraction, field_id: &str) -> String {
    crate::interact::extract_modal_value(&modal.data.components, field_id)
}

fn extract_text_displays(message: &Message) -> Vec<String> {
    let container = message.components.iter().find_map(|c| match c {
        Component::Container(c) => Some(c),
        _ => None,
    });

    let Some(container) = container else {
        return Vec::new();
    };

    container
        .components
        .iter()
        .filter_map(|c| match c {
            serenity::all::ContainerComponent::TextDisplay(td) => td.content.clone(),
            _ => None,
        })
        .collect()
}

fn extract_attachment_prompt_idx(message: &Message) -> Option<usize> {
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
                        let id: &str = custom_id.as_str();
                        if let Some(rest) = id.strip_prefix("review_cancel_attachment:") {
                            return rest.split(':').next()?.parse().ok();
                        }
                    }
                }
            }
        }
    }
    None
}

fn parse_state_from_message(message: &Message) -> Option<SubmissionState> {
    let container = message.components.iter().find_map(|c| match c {
        Component::Container(c) => Some(c),
        _ => None,
    })?;

    let texts = extract_text_displays(message);

    let submitter_id = texts.iter().find_map(|t| {
        let start = t.find("<@")? + 2;
        let end = t[start..].find('>')? + start;
        t[start..end].parse::<u64>().ok()
    })?;

    let mut players = Vec::new();

    for part in &*container.components {
        match part {
            serenity::all::ContainerComponent::TextDisplay(td) => {
                let Some(content) = &td.content else { continue };
                let trimmed = content.trim();

                if is_player_entry(trimmed) {
                    if let Some(player) = parse_player_block(trimmed) {
                        players.push(player);
                    }
                    continue;
                }

                if let Some(player) = players.last_mut() {
                    if let Some(status) = parse_status_line(trimmed) {
                        player.status = status.0;
                        player.reviewer = status.1;
                        player.review_note = status.2;
                    } else if let Some(votes) = parse_votes_line(trimmed) {
                        player.accept_votes = votes.0;
                        player.reject_votes = votes.1;
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
                    if let Some(player) = players.last_mut() {
                        player.evidence.push(Evidence::Attachment { url });
                    }
                }
            }
            _ => {}
        }
    }

    let submitted = texts.iter().any(|t| {
        t.contains("Approved by") || t.contains("Rejected by") || t.contains("awaiting review")
    });

    Some(SubmissionState {
        submitter_id,
        players,
        submitted,
    })
}

fn is_player_entry(text: &str) -> bool {
    let first_line = text.lines().next().unwrap_or("");
    first_line.contains(" \u{2014} `") && first_line.contains('`')
}

fn find_dash_separator(s: &str) -> Option<usize> {
    s.find(" \u{2014} ")
}

fn parse_player_block(content: &str) -> Option<PlayerEntry> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let header = lines[0];
    let username = header.split('`').nth(1)?.to_string();

    let dash_pos = find_dash_separator(header)?;
    let tag_part = &header[..dash_pos];
    let display_name = if tag_part.contains('>') {
        tag_part.split('>').next_back()?.trim()
    } else {
        tag_part.trim()
    };
    let tag_name = lookup_tag_name_from_display(display_name)?;

    let reason = lines
        .get(1)
        .and_then(|l| l.strip_prefix("> "))
        .unwrap_or("")
        .to_string();

    let meta_line = lines.get(2).unwrap_or(&"");
    let meta = meta_line.strip_prefix("> -# ").unwrap_or(meta_line);

    let is_nicked = meta.contains("Nicked");
    let uuid = if is_nicked {
        String::new()
    } else {
        meta.strip_prefix("UUID: ")
            .and_then(|s| s.split_whitespace().next())
            .unwrap_or("")
            .replace('-', "")
    };

    let evidence = lines
        .iter()
        .skip(3)
        .filter_map(|l| parse_evidence_line(l.trim()))
        .collect();

    Some(PlayerEntry {
        username,
        uuid,
        tag_type: tag_name.to_string(),
        reason,
        is_nicked,
        status: PlayerStatus::Pending,
        reviewer: None,
        review_note: None,
        evidence,
        conflict_warning: None,
        accept_votes: Vec::new(),
        reject_votes: Vec::new(),
    })
}

fn parse_status_line(text: &str) -> Option<(PlayerStatus, Option<String>, Option<String>)> {
    let line = text.strip_prefix("-# ")?;

    if line.contains("Pending") {
        return Some((PlayerStatus::Pending, None, None));
    }

    if let Some(rest) = line.strip_prefix("Approved by ") {
        return Some((PlayerStatus::Approved, Some(rest.to_string()), None));
    }

    if let Some(rest) = line.strip_prefix("Rejected by ") {
        let (reviewer, note) = if let Some(colon_pos) = rest.find(": \"") {
            let reviewer = rest[..colon_pos].to_string();
            let note = rest[colon_pos + 3..]
                .strip_suffix('"')
                .unwrap_or(&rest[colon_pos + 3..])
                .to_string();
            (reviewer, Some(note))
        } else {
            (rest.to_string(), None)
        };
        return Some((PlayerStatus::Rejected, Some(reviewer), note));
    }

    None
}

fn parse_votes_line(text: &str) -> Option<(Vec<u64>, Vec<u64>)> {
    let line = text.strip_prefix("-# Votes: ")?;
    let mut accepts = Vec::new();
    let mut rejects = Vec::new();

    for token in line.split_whitespace() {
        if let Some(id_str) = token.strip_prefix('+') {
            if let Ok(id) = id_str.parse::<u64>() {
                accepts.push(id);
            }
        } else if let Some(id_str) = token.strip_prefix('-') {
            if let Ok(id) = id_str.parse::<u64>() {
                rejects.push(id);
            }
        }
    }

    if accepts.is_empty() && rejects.is_empty() {
        return None;
    }

    Some((accepts, rejects))
}

fn lookup_tag_name_from_display(display: &str) -> Option<&'static str> {
    blacklist::all()
        .iter()
        .find(|t| t.display_name == display)
        .map(|t| t.name)
}

fn parse_evidence_line(line: &str) -> Option<Evidence> {
    let line = line.strip_prefix("- ").unwrap_or(line);

    if line.starts_with("`/replay") {
        let command = line.split('`').nth(1)?;
        let replay = parse_replay(command)?;
        let note = line
            .split("Note: \"")
            .nth(1)
            .and_then(|s| s.strip_suffix('"'))
            .map(|s| s.to_string());
        Some(Evidence::Replay { replay, note })
    } else {
        None
    }
}

fn build_header(state: &SubmissionState) -> CreateContainerComponent<'static> {
    text(format!(
        "## {} Tag Review\n-# Submitted by <@{}>",
        EMOTE_TAG, state.submitter_id
    ))
}

fn build_review_message(state: &SubmissionState) -> Vec<CreateComponent<'static>> {
    let id = state.submitter_id;
    let mut parts = vec![build_header(state), separator()];

    if state.players.is_empty() {
        parts.push(text("-# No players added yet"));
    }

    for (idx, player) in state.players.iter().enumerate() {
        parts.push(text(render_player_block(player)));

        if let Some(gallery) = media_gallery_for(player) {
            parts.push(gallery);
        }

        if state.submitted {
            if player.status == PlayerStatus::Pending {
                if has_votes(player) {
                    parts.push(text(render_vote_status(player)));
                }

                parts.push(CreateContainerComponent::ActionRow(
                    CreateActionRow::Buttons(
                        vec![
                            CreateButton::new(format!("review_approve:{}:{}", idx, id))
                                .label("Accept")
                                .style(ButtonStyle::Success),
                            CreateButton::new(format!("review_reject:{}:{}", idx, id))
                                .label("Reject")
                                .style(ButtonStyle::Danger),
                        ]
                        .into(),
                    ),
                ));
            } else {
                parts.push(text(render_status_line(player)));
            }
        } else {
            let edit_select = CreateSelectMenu::new(
                format!("review_tag_select_edit:{idx}:{id}"),
                CreateSelectMenuKind::String {
                    options: build_tag_select_options(Some(&player.tag_type)).into(),
                },
            )
            .placeholder("Change tag type");

            parts.push(CreateContainerComponent::ActionRow(
                CreateActionRow::SelectMenu(edit_select),
            ));
            parts.push(CreateContainerComponent::ActionRow(
                CreateActionRow::Buttons(
                    vec![
                        CreateButton::new(format!("review_add_replay:{idx}:{id}"))
                            .label("Add Replay")
                            .style(ButtonStyle::Secondary),
                        CreateButton::new(format!("review_add_attachment:{idx}:{id}"))
                            .label("Add Media")
                            .style(ButtonStyle::Secondary),
                        CreateButton::new(format!("review_remove_player:{idx}:{id}"))
                            .label("Remove")
                            .style(ButtonStyle::Danger),
                    ]
                    .into(),
                ),
            ));
        }

        parts.push(separator());
    }

    if state.submitted {
        parts.push(text("-# Submitted \u{2014} awaiting review"));

        let has_pending = state
            .players
            .iter()
            .any(|p| p.status == PlayerStatus::Pending);
        if has_pending {
            parts.push(CreateContainerComponent::ActionRow(
                CreateActionRow::Buttons(
                    vec![
                        CreateButton::new(format!("review_edit_submitted:{id}"))
                            .label("Edit")
                            .style(ButtonStyle::Secondary),
                    ]
                    .into(),
                ),
            ));
        }
    } else {
        if state.players.len() < 4 {
            let add_select = CreateSelectMenu::new(
                format!("review_tag_select_add:{id}"),
                CreateSelectMenuKind::String {
                    options: build_tag_select_options(None).into(),
                },
            )
            .placeholder("Add Player \u{2014} select tag type");

            parts.push(CreateContainerComponent::ActionRow(
                CreateActionRow::SelectMenu(add_select),
            ));
        }
        parts.push(separator());
        parts.push(CreateContainerComponent::ActionRow(
            CreateActionRow::Buttons(
                vec![
                    CreateButton::new(format!("review_submit:{id}"))
                        .label("Submit for Review")
                        .style(ButtonStyle::Success),
                    CreateButton::new(format!("review_cancel_thread:{id}"))
                        .label("Cancel")
                        .style(ButtonStyle::Danger),
                ]
                .into(),
            ),
        ));
    }

    let container = CreateContainer::new(parts);
    vec![CreateComponent::Container(container)]
}

fn render_player_block(player: &PlayerEntry) -> String {
    let def = lookup_tag(&player.tag_type);
    let emote = def.map(|d| d.emote).unwrap_or("");
    let display_name = def.map(|d| d.display_name).unwrap_or(&player.tag_type);

    let uuid_line = if player.is_nicked {
        "Nicked \u{2014} UUID could not be resolved".to_string()
    } else {
        format!("UUID: {}", format_uuid_dashed(&player.uuid))
    };

    let mut block = format!(
        "{} {} \u{2014} `{}`\n> {}\n> -# {}",
        emote,
        display_name,
        player.username,
        sanitize_reason(&player.reason),
        uuid_line,
    );

    let replays: Vec<String> = player
        .evidence
        .iter()
        .filter_map(|e| match e {
            Evidence::Replay { replay, note } => Some(render_replay_line(replay, note.as_deref())),
            _ => None,
        })
        .collect();

    if !replays.is_empty() {
        block.push('\n');
        block.push_str(&replays.join("\n"));
    }

    let media_count = player
        .evidence
        .iter()
        .filter(|e| matches!(e, Evidence::Attachment { .. }))
        .count();

    if media_count > 0 {
        block.push_str(&format!(
            "\n-# {} media attachment{}",
            media_count,
            if media_count == 1 { "" } else { "s" }
        ));
    }

    if let Some(warning) = &player.conflict_warning {
        block.push('\n');
        block.push_str(warning);
    }

    block
}

fn media_gallery_for(player: &PlayerEntry) -> Option<CreateContainerComponent<'static>> {
    let items: Vec<CreateMediaGalleryItem> = player
        .evidence
        .iter()
        .filter_map(|e| match e {
            Evidence::Attachment { url, .. } => Some(CreateMediaGalleryItem::new(
                CreateUnfurledMediaItem::new(url.clone()),
            )),
            _ => None,
        })
        .collect();

    if items.is_empty() {
        return None;
    }

    Some(CreateContainerComponent::MediaGallery(
        CreateMediaGallery::new(items),
    ))
}

fn render_status_line(player: &PlayerEntry) -> String {
    match &player.status {
        PlayerStatus::Pending => "-# Pending review".to_string(),
        PlayerStatus::Approved => "-# Approved".to_string(),
        PlayerStatus::Rejected => match &player.review_note {
            Some(note) => format!("-# Rejected: \"{note}\""),
            None => "-# Rejected".to_string(),
        },
    }
}

fn render_vote_status(player: &PlayerEntry) -> String {
    let accepts = player.accept_votes.len();
    let rejects = player.reject_votes.len();
    let has_disagreement = accepts > 0 && rejects > 0;

    if has_disagreement {
        format!(
            "-# {} accept, {} reject \u{2014} staff required",
            accepts, rejects
        )
    } else if accepts > 0 {
        format!("-# {}/3 accepting", accepts)
    } else {
        format!("-# {}/3 rejecting", rejects)
    }
}

fn has_votes(player: &PlayerEntry) -> bool {
    !player.accept_votes.is_empty() || !player.reject_votes.is_empty()
}

fn render_replay_line(replay: &Replay, note: Option<&str>) -> String {
    match note {
        Some(n) => format!("- `{}` \u{2014} Note: \"{}\"", replay.format_command(), n),
        None => format!("- `{}`", replay.format_command()),
    }
}

fn build_vote_message(
    voter_id: u64,
    vote_type: &str,
    tag_type: &str,
    username: &str,
    accept_count: usize,
    reject_count: usize,
) -> CreateMessage<'static> {
    let def = lookup_tag(tag_type);
    let emote = def.map(|d| d.emote).unwrap_or("");
    let display_name = def.map(|d| d.display_name).unwrap_or(tag_type);

    let has_disagreement = accept_count > 0 && reject_count > 0;
    let total = if vote_type == "accept" {
        accept_count
    } else {
        reject_count
    };

    let mut content = format!(
        "<@{voter_id}> voted to **{vote_type}** the {emote} **{display_name}** tag on `{username}`. [{total}/3]"
    );

    if has_disagreement {
        content.push_str(&format!(
            "\n-# {accept_count} accept, {reject_count} reject \u{2014} staff required to resolve"
        ));
    }

    let container = CreateContainer::new(vec![text(content)]);

    CreateMessage::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(vec![CreateComponent::Container(container)])
}

const CONFIRMABLE_TAGS: &[&str] = &["closet_cheater", "blatant_cheater"];

struct ForumTags {
    pending: Option<ForumTagId>,
    approved: Option<ForumTagId>,
    rejected: Option<ForumTagId>,
    nicked: Option<ForumTagId>,
    awaiting_evidence: Option<ForumTagId>,
}

async fn resolve_forum_tags(ctx: &Context, data: &Data) -> ForumTags {
    let empty = ForumTags {
        pending: None,
        approved: None,
        rejected: None,
        nicked: None,
        awaiting_evidence: None,
    };

    let Some(forum_id) = data.review_forum_id else {
        return empty;
    };

    let Ok(channel) = ctx.http.get_channel(forum_id.into()).await else {
        return empty;
    };

    let serenity::all::Channel::Guild(gc) = channel else {
        return empty;
    };

    let find = |name: &str| {
        gc.available_tags
            .iter()
            .find(|t| t.name == name)
            .map(|t| t.id)
    };

    ForumTags {
        pending: find(TAG_PENDING),
        approved: find(TAG_APPROVED),
        rejected: find(TAG_REJECTED),
        nicked: find(TAG_NICKED),
        awaiting_evidence: find(TAG_AWAITING_EVIDENCE),
    }
}

async fn set_forum_tags(ctx: &Context, thread_id: ThreadId, tag_ids: &[ForumTagId]) -> Result<()> {
    thread_id
        .edit(&ctx.http, EditThread::new().applied_tags(tag_ids.to_vec()))
        .await?;
    Ok(())
}

pub async fn create_submission(
    ctx: &Context,
    data: &Data,
    submitter_id: u64,
    player_name: &str,
    player_uuid: &str,
    tag_type: &str,
    reason: &str,
    is_nicked: bool,
) -> Result<ThreadId> {
    let Some(forum_id) = data.review_forum_id else {
        anyhow::bail!("Review forum channel not configured");
    };

    let def = lookup_tag(tag_type);
    let display_name = def.map(|d| d.display_name).unwrap_or(tag_type);

    let player = PlayerEntry {
        username: player_name.to_string(),
        uuid: player_uuid.to_string(),
        tag_type: tag_type.to_string(),
        reason: reason.to_string(),
        is_nicked,
        status: PlayerStatus::Pending,
        reviewer: None,
        review_note: None,
        evidence: Vec::new(),
        conflict_warning: None,
        accept_votes: Vec::new(),
        reject_votes: Vec::new(),
    };

    let state = SubmissionState {
        submitter_id,
        players: vec![player],
        submitted: false,
    };

    let thread_title = format!("{} — {}", player_name, display_name);

    let message = CreateMessage::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(build_review_message(&state));

    let mut forum_post = CreateForumPost::new(thread_title, message);

    let tags = resolve_forum_tags(ctx, data).await;
    if let Some(tag_id) = tags.awaiting_evidence {
        forum_post = forum_post.add_applied_tag(tag_id);
    }
    if is_nicked {
        if let Some(tag_id) = tags.nicked {
            forum_post = forum_post.add_applied_tag(tag_id);
        }
    }

    let thread = forum_id.create_forum_post(&ctx.http, forum_post).await?;

    Ok(thread.id)
}

async fn check_overwrite_conflict(
    data: &Data,
    uuid: &str,
    tag_type: &str,
) -> Result<Option<String>> {
    let repo = BlacklistRepository::new(data.db.pool());
    let existing_tags = repo.get_tags(uuid).await?;
    let new_priority = lookup_tag(tag_type).map(|d| d.priority).unwrap_or(0);

    let conflict = existing_tags
        .iter()
        .find(|t| lookup_tag(&t.tag_type).map(|d| d.priority).unwrap_or(0) == new_priority);

    if let Some(tag) = conflict {
        let def = lookup_tag(&tag.tag_type);
        let emote = def.map(|d| d.emote).unwrap_or("");
        let display = def.map(|d| d.display_name).unwrap_or(&tag.tag_type);
        Ok(Some(format!(
            "⚠ Existing tag: {} {} — \"{}\"",
            emote,
            display,
            sanitize_reason(&tag.reason)
        )))
    } else {
        Ok(None)
    }
}

fn parse_component_ids(custom_id: &str) -> (usize, u64) {
    let mut parts = custom_id.split(':');
    let _ = parts.next();
    let player_idx = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let submitter_id = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (player_idx, submitter_id)
}

fn parse_submitter_id(custom_id: &str) -> Option<u64> {
    custom_id.split(':').last()?.parse().ok()
}

fn verify_submitter(component: &ComponentInteraction, custom_id: &str) -> bool {
    let expected = parse_submitter_id(custom_id).unwrap_or(0);
    component.user.id.get() == expected
}

async fn send_vote_error(
    ctx: &Context,
    component: &ComponentInteraction,
    message: &str,
) -> Result<()> {
    component
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

fn get_builder_message(component: &ComponentInteraction) -> Message {
    *component.message.clone()
}

async fn find_builder_message(ctx: &Context, channel_id: GenericChannelId) -> Option<Message> {
    let thread_id = MessageId::new(channel_id.get());
    ctx.http.get_message(channel_id, thread_id).await.ok()
}

async fn send_thread_message(
    ctx: &Context,
    channel_id: GenericChannelId,
    content: &str,
) -> Result<()> {
    let msg = CreateMessage::new().content(content);
    ctx.http
        .send_message(channel_id, Vec::<CreateAttachment>::new(), &msg)
        .await?;
    Ok(())
}

async fn update_builder(
    ctx: &Context,
    channel_id: GenericChannelId,
    message: &Message,
    state: &SubmissionState,
) -> Result<()> {
    let edit = EditMessage::new()
        .content("")
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(build_review_message(state));

    ctx.http
        .edit_message(channel_id, message.id, &edit, Vec::new())
        .await?;

    Ok(())
}

async fn update_builder_with_files(
    ctx: &Context,
    channel_id: GenericChannelId,
    message: &Message,
    state: &SubmissionState,
    files: Vec<CreateAttachment<'static>>,
) -> Result<()> {
    let mut edit = EditMessage::new()
        .content("")
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(build_review_message(state));

    let mut attachments = EditAttachments::keep_all(message);
    for f in &files {
        attachments = attachments.add(f.clone());
    }
    edit = edit.attachments(attachments);

    ctx.http
        .edit_message(channel_id, message.id, &edit, files)
        .await?;

    Ok(())
}

pub async fn handle_tag_select_add(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can use these buttons")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let tag_type = match &component.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => {
            values.first().map(|s| s.as_str()).unwrap_or("")
        }
        _ => return Ok(()),
    };

    if lookup_tag(tag_type).is_none() {
        return Ok(());
    }

    let submitter_id = parse_submitter_id(&component.data.custom_id).unwrap_or(0);

    let player_input = CreateInputText::new(InputTextStyle::Short, "player")
        .placeholder("Minecraft username")
        .min_length(1)
        .max_length(16);
    let player_label = CreateLabel::input_text("Player Name", player_input);

    let reason_input = CreateInputText::new(InputTextStyle::Short, "reason")
        .placeholder("Reason for this tag")
        .min_length(1)
        .max_length(120);
    let reason_label = CreateLabel::input_text("Reason", reason_input);

    let modal = CreateModal::new(
        format!("review_player_modal:{tag_type}:{submitter_id}"),
        "Add Player",
    )
    .components(vec![
        CreateModalComponent::Label(player_label),
        CreateModalComponent::Label(reason_label),
    ]);

    component
        .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}

pub async fn handle_player_modal(
    ctx: &Context,
    modal: &ModalInteraction,
    data: &Data,
) -> Result<()> {
    modal.defer_ephemeral(&ctx.http).await?;

    let custom_id = modal
        .data
        .custom_id
        .strip_prefix("review_player_modal:")
        .unwrap_or("");
    let tag_type = custom_id.split(':').next().unwrap_or("").to_string();
    if lookup_tag(&tag_type).is_none() {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new()
                    .content(format!("Unknown tag type: `{}`", tag_type)),
            )
            .await?;
        return Ok(());
    }

    let player_name = extract_modal_value(modal, "player");
    let reason = extract_modal_value(modal, "reason");

    let (resolved_name, resolved_uuid, is_nicked) = match data.api.resolve(&player_name).await {
        Ok(info) => (info.username, info.uuid, false),
        Err(_) => (player_name, String::new(), true),
    };

    if resolved_uuid.is_empty() && !is_nicked {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new().content("Player not found"),
            )
            .await?;
        return Ok(());
    }

    let channel_id = modal.channel_id;

    let Some(builder_msg) = find_builder_message(ctx, channel_id).await else {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new()
                    .content("Could not find the submission message"),
            )
            .await?;
        return Ok(());
    };

    let Some(mut state) = parse_state_from_message(&builder_msg) else {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new()
                    .content("Could not parse submission state"),
            )
            .await?;
        return Ok(());
    };

    let already_added = state.players.iter().any(|p| {
        if is_nicked {
            p.is_nicked && p.username.eq_ignore_ascii_case(&resolved_name)
        } else {
            p.uuid == resolved_uuid
        }
    });

    if already_added {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new()
                    .content(format!("`{}` is already in this submission", resolved_name)),
            )
            .await?;
        return Ok(());
    }

    let new_player = PlayerEntry {
        username: resolved_name,
        uuid: if is_nicked {
            String::new()
        } else {
            resolved_uuid.clone()
        },
        tag_type,
        reason,
        is_nicked,
        status: PlayerStatus::Pending,
        reviewer: None,
        review_note: None,
        evidence: Vec::new(),
        conflict_warning: None,
        accept_votes: Vec::new(),
        reject_votes: Vec::new(),
    };

    state.players.push(new_player);

    update_builder(ctx, channel_id, &builder_msg, &state).await?;

    if is_nicked {
        let tags = resolve_forum_tags(ctx, data).await;
        if let Some(tag_id) = tags.nicked {
            let mut current_tags = Vec::new();
            if let Some(aw) = tags.awaiting_evidence {
                current_tags.push(aw);
            }
            current_tags.push(tag_id);
            let thread_id = modal.channel_id.expect_thread();
            let _ = set_forum_tags(ctx, thread_id, &current_tags).await;
        }
    }

    let _ = modal.delete_response(&ctx.http).await;

    Ok(())
}

pub async fn handle_add_replay(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can use these buttons")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let (player_idx, submitter_id) = parse_component_ids(&component.data.custom_id);

    let replay_input = CreateInputText::new(InputTextStyle::Short, "replay")
        .placeholder("/replay 9f2fa87d-ed0b-471b-a2e6-cb42777beec8 #9d303f9d")
        .min_length(1)
        .max_length(200);
    let replay_label = CreateLabel::input_text("Replay Command or ID", replay_input);

    let note_input = CreateInputText::new(InputTextStyle::Short, "note")
        .placeholder("Optional note about this replay")
        .required(false)
        .max_length(75);
    let note_label = CreateLabel::input_text("Note (optional)", note_input);

    let modal = CreateModal::new(
        format!("review_replay_modal:{player_idx}:{submitter_id}"),
        "Add Replay Evidence",
    )
    .components(vec![
        CreateModalComponent::Label(replay_label),
        CreateModalComponent::Label(note_label),
    ]);

    component
        .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}

pub async fn handle_replay_modal(
    ctx: &Context,
    modal: &ModalInteraction,
    _data: &Data,
) -> Result<()> {
    modal.defer_ephemeral(&ctx.http).await?;

    let custom_id = modal
        .data
        .custom_id
        .strip_prefix("review_replay_modal:")
        .unwrap_or("");
    let player_idx: usize = custom_id
        .split(':')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let replay_input = extract_modal_value(modal, "replay");
    let note = extract_modal_value(modal, "note");

    let Some(replay) = parse_replay(&replay_input) else {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new().content(
                    "Could not parse replay. Provide a valid replay UUID or `/replay` command",
                ),
            )
            .await?;
        return Ok(());
    };

    let channel_id = modal.channel_id;

    let Some(builder_msg) = find_builder_message(ctx, channel_id).await else {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new()
                    .content("Could not find the submission message"),
            )
            .await?;
        return Ok(());
    };

    let Some(mut state) = parse_state_from_message(&builder_msg) else {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new()
                    .content("Could not parse submission state"),
            )
            .await?;
        return Ok(());
    };

    let Some(player) = state.players.get_mut(player_idx) else {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new().content("Player not found"),
            )
            .await?;
        return Ok(());
    };

    let note = if note.is_empty() { None } else { Some(note) };
    player.evidence.push(Evidence::Replay { replay, note });

    update_builder(ctx, channel_id, &builder_msg, &state).await?;

    let _ = modal.delete_response(&ctx.http).await;

    Ok(())
}

pub async fn handle_add_attachment(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can use these buttons")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let (player_idx, _) = parse_component_ids(&component.data.custom_id);

    let builder_msg = get_builder_message(component);
    let player_name = parse_state_from_message(&builder_msg)
        .and_then(|s| s.players.get(player_idx).map(|p| p.username.clone()))
        .unwrap_or_default();

    let submitter_id = parse_submitter_id(&component.data.custom_id).unwrap_or(0);

    let container = CreateContainer::new(vec![
        text(format!(
            "Upload media evidence for **`{}`** in this thread.",
            player_name
        )),
        CreateContainerComponent::ActionRow(CreateActionRow::Buttons(
            vec![
                CreateButton::new(format!(
                    "review_cancel_attachment:{player_idx}:{submitter_id}"
                ))
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

pub async fn handle_cancel_attachment(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can use these buttons")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let message = component.message.clone();
    component
        .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
        .await?;
    let _ = message.delete(&ctx.http, None).await;
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
    _data: &Data,
) -> Result<()> {
    let attachments = collect_attachment_urls(message);
    if attachments.is_empty() {
        return Ok(());
    }

    let channel_id = message.channel_id;

    let Some(builder_msg) = find_builder_message(ctx, channel_id).await else {
        return Ok(());
    };

    let Some(mut state) = parse_state_from_message(&builder_msg) else {
        return Ok(());
    };

    if state.submitted {
        return Ok(());
    }

    if message.author.id.get() != state.submitter_id {
        return Ok(());
    }

    let messages = ctx
        .http
        .get_messages(channel_id, None, None)
        .await
        .unwrap_or_default();

    let Some((player_idx, prompt_msg_id)) = messages.iter().find_map(|m| {
        if !m.author.bot() {
            return None;
        }
        extract_attachment_prompt_idx(m).map(|idx| (idx, m.id))
    }) else {
        return Ok(());
    };

    let Some(player) = state.players.get_mut(player_idx) else {
        return Ok(());
    };

    let existing_count = player
        .evidence
        .iter()
        .filter(|e| matches!(e, Evidence::Attachment { .. }))
        .count();

    let remaining = MAX_MEDIA_PER_PLAYER.saturating_sub(existing_count);
    if remaining == 0 {
        let _ = send_thread_message(
            ctx,
            channel_id,
            &format!(
                "Maximum of {} media attachments per player reached.",
                MAX_MEDIA_PER_PLAYER
            ),
        )
        .await;
        let _ = message.delete(&ctx.http, None).await;
        let _ = ctx
            .http
            .delete_message(channel_id, prompt_msg_id, None)
            .await;
        return Ok(());
    }

    let mut files = Vec::new();
    for (i, (url, orig_filename)) in attachments.iter().take(remaining).enumerate() {
        let ext = orig_filename.rsplit('.').next().unwrap_or("png");
        let filename = format!("{}_{}.{}", player.username, existing_count + i + 1, ext);
        let att = CreateAttachment::url(&ctx.http, url.as_str(), filename.clone()).await?;
        files.push(att);
        player.evidence.push(Evidence::Attachment {
            url: format!("attachment://{filename}"),
        });
    }

    update_builder_with_files(ctx, channel_id, &builder_msg, &state, files).await?;

    let _ = message.delete(&ctx.http, None).await;
    let _ = ctx
        .http
        .delete_message(channel_id, prompt_msg_id, None)
        .await;

    Ok(())
}

pub async fn handle_remove_player(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can use these buttons")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let (player_idx, _) = parse_component_ids(&component.data.custom_id);

    let channel_id = component.channel_id;
    let Some(builder_msg) = find_builder_message(ctx, channel_id).await else {
        return Ok(());
    };

    let Some(mut state) = parse_state_from_message(&builder_msg) else {
        return Ok(());
    };

    if player_idx < state.players.len() {
        state.players.remove(player_idx);
    }

    component
        .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
        .await?;

    update_builder(ctx, channel_id, &builder_msg, &state).await?;

    Ok(())
}

pub async fn handle_remove_evidence(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can use these buttons")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let parts: Vec<&str> = component.data.custom_id.split(':').collect();
    let player_idx: usize = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let ev_idx: usize = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);

    let channel_id = component.channel_id;
    let Some(builder_msg) = find_builder_message(ctx, channel_id).await else {
        return Ok(());
    };

    let Some(mut state) = parse_state_from_message(&builder_msg) else {
        return Ok(());
    };

    if let Some(player) = state.players.get_mut(player_idx) {
        if ev_idx < player.evidence.len() {
            player.evidence.remove(ev_idx);
        }
    }

    component
        .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
        .await?;

    update_builder(ctx, channel_id, &builder_msg, &state).await?;

    Ok(())
}

pub async fn handle_tag_select_edit(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can use these buttons")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let tag_type = match &component.data.kind {
        ComponentInteractionDataKind::StringSelect { values } => {
            values.first().map(|s| s.as_str()).unwrap_or("")
        }
        _ => return Ok(()),
    };

    if lookup_tag(tag_type).is_none() {
        return Ok(());
    }

    let (player_idx, submitter_id) = parse_component_ids(&component.data.custom_id);

    let message = get_builder_message(component);
    let current_reason = parse_state_from_message(&message)
        .and_then(|s| s.players.get(player_idx).map(|p| p.reason.clone()))
        .unwrap_or_default();

    let reason_input = CreateInputText::new(InputTextStyle::Short, "reason")
        .placeholder("Reason for this tag")
        .value(current_reason)
        .min_length(1)
        .max_length(120);
    let reason_label = CreateLabel::input_text("Reason", reason_input);

    let modal = CreateModal::new(
        format!("review_edit_player_modal:{player_idx}:{tag_type}:{submitter_id}"),
        "Edit Player",
    )
    .components(vec![CreateModalComponent::Label(reason_label)]);

    component
        .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}

pub async fn handle_edit_player_modal(
    ctx: &Context,
    modal: &ModalInteraction,
    _data: &Data,
) -> Result<()> {
    modal.defer_ephemeral(&ctx.http).await?;

    let custom_id = modal
        .data
        .custom_id
        .strip_prefix("review_edit_player_modal:")
        .unwrap_or("");
    let parts: Vec<&str> = custom_id.split(':').collect();
    let player_idx: usize = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
    let tag_type = parts.get(1).unwrap_or(&"").to_string();

    if lookup_tag(&tag_type).is_none() {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new()
                    .content(format!("Unknown tag type: `{}`", tag_type)),
            )
            .await?;
        return Ok(());
    }

    let reason = extract_modal_value(modal, "reason");

    let channel_id = modal.channel_id;
    let Some(builder_msg) = find_builder_message(ctx, channel_id).await else {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new()
                    .content("Could not find the submission message"),
            )
            .await?;
        return Ok(());
    };

    let Some(mut state) = parse_state_from_message(&builder_msg) else {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new()
                    .content("Could not parse submission state"),
            )
            .await?;
        return Ok(());
    };

    let Some(player) = state.players.get_mut(player_idx) else {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new().content("Player not found"),
            )
            .await?;
        return Ok(());
    };

    player.tag_type = tag_type;
    player.reason = reason;

    update_builder(ctx, channel_id, &builder_msg, &state).await?;

    let _ = modal.delete_response(&ctx.http).await;

    Ok(())
}

pub async fn handle_edit_submitted(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can edit")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let message = get_builder_message(component);
    let Some(mut state) = parse_state_from_message(&message) else {
        return Ok(());
    };

    state.submitted = false;

    component
        .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
        .await?;

    let message = get_builder_message(component);
    update_builder(ctx, component.channel_id, &message, &state).await?;

    Ok(())
}

pub async fn handle_submit(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can use these buttons")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let message = get_builder_message(component);
    let Some(mut state) = parse_state_from_message(&message) else {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Could not parse submission state")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    };

    if state.players.is_empty() {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Add at least one player before submitting")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let has_evidence = state.players.iter().any(|p| !p.evidence.is_empty());
    if !has_evidence {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Add at least one piece of evidence (replay or attachment) before submitting")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    state.submitted = true;

    for player in &mut state.players {
        if !player.is_nicked {
            if let Ok(warning) =
                check_overwrite_conflict(data, &player.uuid, &player.tag_type).await
            {
                player.conflict_warning = warning;
            }
        }
    }

    component
        .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
        .await?;

    let message = get_builder_message(component);
    update_builder(ctx, component.channel_id, &message, &state).await?;

    let tags = resolve_forum_tags(ctx, data).await;
    let mut tag_ids = Vec::new();
    if let Some(id) = tags.pending {
        tag_ids.push(id);
    }
    let has_nicked = state.players.iter().any(|p| p.is_nicked);
    if has_nicked {
        if let Some(id) = tags.nicked {
            tag_ids.push(id);
        }
    }
    let thread_id = component.channel_id.expect_thread();
    let _ = set_forum_tags(ctx, thread_id, &tag_ids).await;

    Ok(())
}

pub async fn handle_approve(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let (player_index, submitter_id) = parse_component_ids(&component.data.custom_id);

    let discord_id = component.user.id.get();
    let rank = super::tag::get_rank(data, discord_id).await?;
    if rank < crate::framework::AccessRank::Member {
        return send_vote_error(
            ctx,
            component,
            "Only members and above can review submissions",
        )
        .await;
    }

    if discord_id == submitter_id {
        return send_vote_error(ctx, component, "You cannot review your own submission").await;
    }

    let message = get_builder_message(component);
    let Some(mut state) = parse_state_from_message(&message) else {
        return Ok(());
    };

    let Some(player) = state.players.get(player_index) else {
        return Ok(());
    };

    if player.status != PlayerStatus::Pending {
        return send_vote_error(ctx, component, "This player has already been reviewed").await;
    }

    if player.accept_votes.contains(&discord_id) || player.reject_votes.contains(&discord_id) {
        return send_vote_error(ctx, component, "You have already voted on this player").await;
    }

    let is_staff = rank >= crate::framework::AccessRank::Helper;

    if !is_staff {
        state.players[player_index].accept_votes.push(discord_id);

        let unanimous = state.players[player_index].reject_votes.is_empty()
            && state.players[player_index].accept_votes.len() >= 3;

        if !unanimous {
            let player = &state.players[player_index];
            let vote_msg = build_vote_message(
                discord_id,
                "accept",
                &player.tag_type,
                &player.username,
                player.accept_votes.len(),
                player.reject_votes.len(),
            );

            let components = build_review_message(&state);
            component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .flags(MessageFlags::IS_COMPONENTS_V2)
                            .components(components),
                    ),
                )
                .await?;

            let channel_id = component.channel_id;
            let _ = ctx
                .http
                .send_message(channel_id.into(), Vec::<CreateAttachment>::new(), &vote_msg)
                .await;

            return Ok(());
        }
    }

    let player = &state.players[player_index];
    let player_uuid = player.uuid.clone();
    let player_tag_type = player.tag_type.clone();
    let player_username = player.username.clone();
    let player_reason = player.reason.clone();

    let media_urls: Vec<String> = player
        .evidence
        .iter()
        .filter_map(|e| match e {
            Evidence::Attachment { url, .. } => Some(url.clone()),
            _ => None,
        })
        .collect();

    let repo = BlacklistRepository::new(data.db.pool());
    let reviewed_by: Vec<i64> = if is_staff {
        vec![discord_id as i64]
    } else {
        state.players[player_index]
            .accept_votes
            .iter()
            .map(|&id| id as i64)
            .collect()
    };

    if !player.is_nicked {
        let existing_tags = repo.get_tags(&player_uuid).await?;
        let new_priority = lookup_tag(&player_tag_type)
            .map(|d| d.priority)
            .unwrap_or(0);
        if let Some(conflict) = existing_tags
            .iter()
            .find(|t| lookup_tag(&t.tag_type).map(|d| d.priority).unwrap_or(0) == new_priority)
        {
            repo.remove_tag(conflict.id, discord_id as i64).await?;
        }

        let reviewed_by_slice = if reviewed_by.is_empty() {
            None
        } else {
            Some(reviewed_by.as_slice())
        };

        let will_confirm =
            !media_urls.is_empty() && CONFIRMABLE_TAGS.contains(&player_tag_type.as_str());
        let stored_type = if will_confirm {
            "confirmed_cheater"
        } else {
            &player_tag_type
        };

        let tag_id = repo
            .add_tag(
                &player_uuid,
                stored_type,
                &player_reason,
                submitter_id as i64,
                false,
                reviewed_by_slice,
            )
            .await?;

        if will_confirm {
            let guild_id = component.guild_id.map(|g| g.get()).unwrap_or(0);
            let review_thread_url = format!(
                "https://discord.com/channels/{}/{}",
                guild_id,
                component.channel_id.get(),
            );
            if let Err(e) = super::evidence::create_evidence_from_review(
                ctx,
                data,
                guild_id,
                &player_uuid,
                &player_username,
                &player_tag_type,
                tag_id,
                &media_urls,
                Some(&review_thread_url),
                discord_id as i64,
            )
            .await
            {
                tracing::error!("Failed to create evidence post: {e:#}");
            }
        }

        let tags = repo.get_tags(&player_uuid).await?;
        if let Some(_new_tag) = tags.iter().find(|t| t.id == tag_id) {
            let event = BlacklistEvent::TagAdded {
                uuid: player_uuid.clone(),
                tag_id,
                added_by: submitter_id as i64,
            };
            data.event_publisher.publish(&event).await;
        }
    }

    let member_repo = MemberRepository::new(data.db.pool());
    let _ = member_repo
        .increment_accepted_tags(submitter_id as i64)
        .await;

    let accurate_ids: Vec<i64> = state.players[player_index]
        .accept_votes
        .iter()
        .map(|&id| id as i64)
        .collect();
    if !accurate_ids.is_empty() {
        let _ = member_repo.increment_accurate_verdicts(&accurate_ids).await;
    }

    state.players[player_index].status = PlayerStatus::Approved;
    state.players[player_index].reviewer = None;
    state.players[player_index].accept_votes.clear();
    state.players[player_index].reject_votes.clear();

    let vote_msg = build_vote_message(
        discord_id,
        "accept",
        &player_tag_type,
        &player_username,
        3,
        0,
    );

    let components = build_review_message(&state);
    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(components),
            ),
        )
        .await?;

    let channel_id = component.channel_id;
    if is_staff {
        let staff_msg = build_vote_message(
            discord_id,
            "accept",
            &player_tag_type,
            &player_username,
            1,
            0,
        );
        let _ = ctx
            .http
            .send_message(
                channel_id.into(),
                Vec::<CreateAttachment>::new(),
                &staff_msg,
            )
            .await;
    } else {
        let _ = ctx
            .http
            .send_message(channel_id.into(), Vec::<CreateAttachment>::new(), &vote_msg)
            .await;
    }

    let _ = send_thread_message(
        ctx,
        channel_id,
        &format!(
            "<@{}> Your tag for `{}` has been **approved**",
            submitter_id, player_username
        ),
    )
    .await;

    let thread_id = component.channel_id.expect_thread();
    check_all_resolved(ctx, data, thread_id, &state).await?;

    Ok(())
}

pub async fn handle_reject(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let (player_index, submitter_id) = parse_component_ids(&component.data.custom_id);

    let discord_id = component.user.id.get();
    let rank = super::tag::get_rank(data, discord_id).await?;
    if rank < crate::framework::AccessRank::Member {
        return send_vote_error(
            ctx,
            component,
            "Only members and above can review submissions",
        )
        .await;
    }

    if discord_id == submitter_id {
        return send_vote_error(ctx, component, "You cannot review your own submission").await;
    }

    let is_staff = rank >= crate::framework::AccessRank::Helper;

    if is_staff {
        let reason_input = CreateInputText::new(InputTextStyle::Short, "reason")
            .placeholder("Why is this submission being rejected?")
            .min_length(1)
            .max_length(30);
        let reason_label = CreateLabel::input_text("Rejection Reason", reason_input);

        let modal = CreateModal::new(
            format!("review_reject_modal:{player_index}:{submitter_id}"),
            "Reject Submission",
        )
        .components(vec![CreateModalComponent::Label(reason_label)]);

        component
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await?;
        return Ok(());
    }

    let message = get_builder_message(component);
    let Some(mut state) = parse_state_from_message(&message) else {
        return Ok(());
    };

    let Some(player) = state.players.get(player_index) else {
        return Ok(());
    };

    if player.status != PlayerStatus::Pending {
        return send_vote_error(ctx, component, "This player has already been reviewed").await;
    }

    if player.accept_votes.contains(&discord_id) || player.reject_votes.contains(&discord_id) {
        return send_vote_error(ctx, component, "You have already voted on this player").await;
    }

    state.players[player_index].reject_votes.push(discord_id);

    let unanimous = state.players[player_index].accept_votes.is_empty()
        && state.players[player_index].reject_votes.len() >= 3;

    if !unanimous {
        let player = &state.players[player_index];
        let vote_msg = build_vote_message(
            discord_id,
            "reject",
            &player.tag_type,
            &player.username,
            player.accept_votes.len(),
            player.reject_votes.len(),
        );

        let components = build_review_message(&state);
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .flags(MessageFlags::IS_COMPONENTS_V2)
                        .components(components),
                ),
            )
            .await?;

        let channel_id = component.channel_id;
        let _ = ctx
            .http
            .send_message(channel_id.into(), Vec::<CreateAttachment>::new(), &vote_msg)
            .await;

        return Ok(());
    }

    let player_tag_type = state.players[player_index].tag_type.clone();
    let player_username = state.players[player_index].username.clone();

    let member_repo = MemberRepository::new(data.db.pool());
    let _ = member_repo
        .increment_rejected_tags(submitter_id as i64)
        .await;

    let accurate_ids: Vec<i64> = state.players[player_index]
        .reject_votes
        .iter()
        .map(|&id| id as i64)
        .collect();
    if !accurate_ids.is_empty() {
        let _ = member_repo.increment_accurate_verdicts(&accurate_ids).await;
    }

    let vote_msg = build_vote_message(
        discord_id,
        "reject",
        &player_tag_type,
        &player_username,
        0,
        3,
    );

    state.players[player_index].status = PlayerStatus::Rejected;
    state.players[player_index].reviewer = None;
    state.players[player_index].accept_votes.clear();
    state.players[player_index].reject_votes.clear();

    let components = build_review_message(&state);
    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(components),
            ),
        )
        .await?;

    let channel_id = component.channel_id;
    let _ = ctx
        .http
        .send_message(channel_id.into(), Vec::<CreateAttachment>::new(), &vote_msg)
        .await;

    let _ = send_thread_message(
        ctx,
        channel_id,
        &format!(
            "<@{}> Your tag for `{}` has been **rejected**",
            submitter_id, player_username
        ),
    )
    .await;

    let thread_id = component.channel_id.expect_thread();
    check_all_resolved(ctx, data, thread_id, &state).await?;

    Ok(())
}

pub async fn handle_reject_modal(
    ctx: &Context,
    modal: &ModalInteraction,
    data: &Data,
) -> Result<()> {
    let custom_id = modal
        .data
        .custom_id
        .strip_prefix("review_reject_modal:")
        .unwrap_or("");
    let parts: Vec<&str> = custom_id.split(':').collect();
    if parts.len() < 2 {
        return Ok(());
    }
    let player_index: usize = parts[0].parse().unwrap_or(0);
    let submitter_id: u64 = parts[1].parse().unwrap_or(0);

    let reason = extract_modal_value(modal, "reason");
    let discord_id = modal.user.id.get();

    modal.defer_ephemeral(&ctx.http).await?;

    let channel_id = modal.channel_id;

    let Some(builder_msg) = find_builder_message(ctx, channel_id).await else {
        return Ok(());
    };

    let Some(mut state) = parse_state_from_message(&builder_msg) else {
        return Ok(());
    };

    let Some(player) = state.players.get(player_index) else {
        return Ok(());
    };

    if player.status != PlayerStatus::Pending {
        modal
            .edit_response(
                &ctx.http,
                serenity::all::EditInteractionResponse::new()
                    .content("This player has already been reviewed"),
            )
            .await?;
        return Ok(());
    }

    let player_username = player.username.clone();
    let player_tag_type = player.tag_type.clone();

    let member_repo = MemberRepository::new(data.db.pool());
    let _ = member_repo
        .increment_rejected_tags(submitter_id as i64)
        .await;

    let accurate_ids: Vec<i64> = state.players[player_index]
        .reject_votes
        .iter()
        .map(|&id| id as i64)
        .collect();
    if !accurate_ids.is_empty() {
        let _ = member_repo.increment_accurate_verdicts(&accurate_ids).await;
    }

    state.players[player_index].status = PlayerStatus::Rejected;
    state.players[player_index].reviewer = None;
    state.players[player_index].review_note = Some(reason.clone());
    state.players[player_index].accept_votes.clear();
    state.players[player_index].reject_votes.clear();

    update_builder(ctx, channel_id, &builder_msg, &state).await?;

    let vote_msg = build_vote_message(
        discord_id,
        "reject",
        &player_tag_type,
        &player_username,
        0,
        1,
    );
    let _ = ctx
        .http
        .send_message(channel_id.into(), Vec::<CreateAttachment>::new(), &vote_msg)
        .await;

    let _ = send_thread_message(
        ctx,
        channel_id,
        &format!(
            "<@{}> Your tag for `{}` has been **rejected**: \"{}\"",
            submitter_id, player_username, reason
        ),
    )
    .await;

    modal
        .edit_response(
            &ctx.http,
            serenity::all::EditInteractionResponse::new().content("Rejected"),
        )
        .await?;

    let thread_id = modal.channel_id.expect_thread();
    check_all_resolved(ctx, data, thread_id, &state).await?;

    Ok(())
}

async fn check_all_resolved(
    ctx: &Context,
    data: &Data,
    thread_id: ThreadId,
    state: &SubmissionState,
) -> Result<()> {
    let all_resolved = state
        .players
        .iter()
        .all(|p| p.status != PlayerStatus::Pending);

    if !all_resolved {
        return Ok(());
    }

    let all_approved = state
        .players
        .iter()
        .all(|p| p.status == PlayerStatus::Approved);
    let all_rejected = state
        .players
        .iter()
        .all(|p| p.status == PlayerStatus::Rejected);

    let tags = resolve_forum_tags(ctx, data).await;
    let mut tag_ids = Vec::new();

    if all_approved {
        if let Some(id) = tags.approved {
            tag_ids.push(id);
        }
    } else if all_rejected {
        if let Some(id) = tags.rejected {
            tag_ids.push(id);
        }
    } else if let Some(id) = tags.pending {
        tag_ids.push(id);
    }

    let has_nicked = state.players.iter().any(|p| p.is_nicked);
    if has_nicked {
        if let Some(id) = tags.nicked {
            tag_ids.push(id);
        }
    }

    let _ = set_forum_tags(ctx, thread_id, &tag_ids).await;

    let channel_id: GenericChannelId = thread_id.into();
    let _ = send_thread_message(
        ctx,
        channel_id,
        "All players have been reviewed. This thread is now closed.",
    )
    .await;

    let _ = thread_id
        .edit(&ctx.http, EditThread::new().archived(true).locked(true))
        .await;

    Ok(())
}

pub fn build_confirmation_message(
    submitter_id: u64,
    player_name: &str,
    player_uuid: &str,
    tag_type: &str,
    reason: &str,
    is_nicked: bool,
) -> Vec<CreateComponent<'static>> {
    let def = lookup_tag(tag_type);
    let emote = def.map(|d| d.emote).unwrap_or("");
    let display_name = def.map(|d| d.display_name).unwrap_or(tag_type);

    let confirm_id = format!(
        "review_confirm:{submitter_id}:{tag_type}:{}:{is_nicked}",
        if player_uuid.is_empty() {
            "none"
        } else {
            player_uuid
        }
    );

    let mut parts: Vec<CreateContainerComponent> = vec![text(format!(
        "## {} Confirm Submission\n{} {} \u{2014} `{}`\n> {}",
        EMOTE_ADDTAG,
        emote,
        display_name,
        player_name,
        sanitize_reason(reason),
    ))];

    if is_nicked {
        parts.push(text(
            "**This player could not be resolved.** By confirming, you acknowledge this player will be tagged as a **nick**."
        ));
    }

    parts.push(text(
        "-# You do not have permission to directly apply this tag. A review thread will be created for mod approval.",
    ));

    parts.push(separator());
    parts.push(CreateContainerComponent::ActionRow(
        CreateActionRow::Buttons(
            vec![
                CreateButton::new(confirm_id)
                    .label("Confirm")
                    .style(ButtonStyle::Success),
                CreateButton::new(format!("review_cancel:{submitter_id}"))
                    .label("Cancel")
                    .style(ButtonStyle::Secondary),
            ]
            .into(),
        ),
    ));

    let container = CreateContainer::new(parts);
    vec![CreateComponent::Container(container)]
}

struct ConfirmationData {
    player_name: String,
    player_uuid: String,
    tag_type: String,
    reason: String,
    is_nicked: bool,
}

fn parse_confirmation_data(custom_id: &str, message: &Message) -> Option<ConfirmationData> {
    let stripped = custom_id.strip_prefix("review_confirm:")?;
    let parts: Vec<&str> = stripped.splitn(4, ':').collect();
    if parts.len() < 4 {
        return None;
    }
    let tag_type = parts[1].to_string();
    let player_uuid = if parts[2] == "none" {
        String::new()
    } else {
        parts[2].to_string()
    };
    let is_nicked = parts[3] == "true";

    let texts = extract_text_displays(message);
    let preview = texts.iter().find(|t| t.contains(" \u{2014} `"))?;

    let player_name = preview.split('`').nth(1)?.to_string();
    let reason = preview.split("\n> ").nth(1).unwrap_or("").to_string();

    Some(ConfirmationData {
        player_name,
        player_uuid,
        tag_type,
        reason,
        is_nicked,
    })
}

pub async fn handle_confirm(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let message = component.message.clone();
    let Some(conf) = parse_confirmation_data(&component.data.custom_id, &message) else {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Could not parse confirmation data")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    };

    let submitter_id = component.user.id.get();

    match create_submission(
        ctx,
        data,
        submitter_id,
        &conf.player_name,
        &conf.player_uuid,
        &conf.tag_type,
        &conf.reason,
        conf.is_nicked,
    )
    .await
    {
        Ok(thread_id) => {
            spawn_submission_timeout(ctx.clone(), thread_id);

            let container = CreateContainer::new(vec![text(format!(
                "## {} Tag Review Created\nYour submission has been created in <#{}>.\nAdd evidence and submit for mod review.",
                EMOTE_ADDTAG, thread_id
            ))]);

            component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .flags(MessageFlags::IS_COMPONENTS_V2)
                            .components(vec![CreateComponent::Container(container)]),
                    ),
                )
                .await?;
        }
        Err(e) => {
            tracing::error!("Failed to create review submission: {}", e);
            component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                            .flags(MessageFlags::IS_COMPONENTS_V2)
                            .components(vec![CreateComponent::Container(CreateContainer::new(
                                vec![text("## Error\nFailed to create review submission")],
                            ))]),
                    ),
                )
                .await?;
        }
    }

    Ok(())
}

pub async fn handle_cancel(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    let container = CreateContainer::new(vec![text(format!(
        "## {} Submission Cancelled",
        EMOTE_TAG,
    ))]);

    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(vec![CreateComponent::Container(container)]),
            ),
        )
        .await?;

    Ok(())
}

pub async fn handle_cancel_thread(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can use these buttons")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let submitter_id = component.data.custom_id.split(':').last().unwrap_or("0");

    component
        .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
        .await?;

    let channel_id: GenericChannelId = component.channel_id.into();

    let delete_msg = CreateMessage::new()
        .content("Deleting post in 30 seconds.")
        .components(vec![CreateComponent::ActionRow(CreateActionRow::Buttons(
            vec![
                CreateButton::new(format!("review_abort_delete:{submitter_id}"))
                    .label("Cancel")
                    .style(ButtonStyle::Secondary),
            ]
            .into(),
        ))]);

    let sent = ctx
        .http
        .send_message(channel_id, Vec::<CreateAttachment>::new(), &delete_msg)
        .await?;

    let http = ctx.http.clone();
    let msg_id = sent.id;

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;

        let Ok(msg) = http.get_message(channel_id, msg_id).await else {
            return;
        };
        if msg.content != "Deleting post in 30 seconds." {
            return;
        }

        let _ = channel_id.delete(&http, None).await;
    });

    Ok(())
}

pub async fn handle_abort_delete(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    if !verify_submitter(component, &component.data.custom_id) {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only the submission creator can use these buttons")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let channel_id: GenericChannelId = component.channel_id.into();
    let _ = ctx
        .http
        .delete_message(channel_id, component.message.id, None)
        .await;

    component
        .create_response(&ctx.http, CreateInteractionResponse::Acknowledge)
        .await
        .ok();

    Ok(())
}

pub fn spawn_submission_timeout(ctx: Context, thread_id: ThreadId) {
    let channel_id: GenericChannelId = thread_id.into();

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(SUBMISSION_WARNING_SECS)).await;

        let Some(msg) = find_builder_message(&ctx, channel_id).await else {
            return;
        };
        let Some(state) = parse_state_from_message(&msg) else {
            return;
        };
        if state.submitted {
            return;
        }

        let _ = send_thread_message(
            &ctx,
            channel_id,
            "This submission will be automatically cancelled in 10 minutes due to inactivity.",
        )
        .await;

        let remaining = SUBMISSION_TIMEOUT_SECS - SUBMISSION_WARNING_SECS;
        tokio::time::sleep(std::time::Duration::from_secs(remaining)).await;

        let Some(msg) = find_builder_message(&ctx, channel_id).await else {
            return;
        };
        let Some(state) = parse_state_from_message(&msg) else {
            return;
        };
        if state.submitted {
            return;
        }

        let _ = channel_id.delete(&ctx.http, None).await;
    });
}
