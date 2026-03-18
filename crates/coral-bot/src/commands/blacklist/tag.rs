use anyhow::Result;
use blacklist::{EMOTE_ADDTAG, EMOTE_EDITTAG, EMOTE_REMOVETAG, EMOTE_TAG, lookup as lookup_tag};
use database::{BlacklistRepository, CacheRepository, MemberRepository};
use serenity::all::{
    ButtonStyle, CommandInteraction, CommandOptionType, ComponentInteraction, Context,
    CreateActionRow, CreateAttachment, CreateButton, CreateCommand, CreateCommandOption,
    CreateComponent, CreateContainer, CreateContainerComponent, CreateInputText,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateLabel, CreateModal,
    CreateModalComponent, CreateSection, CreateSectionAccessory, CreateSectionComponent,
    CreateSelectMenuOption, CreateSeparator, CreateTextDisplay, CreateThumbnail,
    CreateUnfurledMediaItem, EditInteractionResponse, InputTextStyle, MessageFlags,
    ResolvedOption, ResolvedValue,
};

use super::channel::{
    COLOR_DANGER, COLOR_FALLBACK, COLOR_INFO, COLOR_SUCCESS, format_added_line,
    post_lock_change, post_new_tag, post_overwritten_tag, post_tag_changed, post_tag_removed,
};
use crate::framework::{AccessRank, Data};
use crate::utils::{format_uuid_dashed, sanitize_reason};

const FACE_SIZE: u32 = 128;
const FACE_FILENAME: &str = "face.png";
const EMOTE_EVIDENCE: &str = "<:evidencefound:1482666860225888346>";
const EMOTE_NO_EVIDENCE: &str = "<:noevidence:1482666258938990696>";

fn face_thumbnail() -> CreateThumbnail<'static> {
    CreateThumbnail::new(CreateUnfurledMediaItem::new(format!(
        "attachment://{}",
        FACE_FILENAME
    )))
}

async fn face_attachment(data: &Data, uuid: &str) -> Option<CreateAttachment<'static>> {
    let png = data.skin_provider.fetch_face(uuid, FACE_SIZE).await?;
    Some(CreateAttachment::bytes(png, FACE_FILENAME))
}

pub struct PendingOverwrite {
    pub uuid: String,
    pub old_tag_id: i64,
    pub tag_type: String,
    pub reason: String,
    pub hide: bool,
}

fn tag_choices(option: CreateCommandOption<'static>) -> CreateCommandOption<'static> {
    option
        .add_string_choice("Sniper", "sniper")
        .add_string_choice("Blatant Cheater", "blatant_cheater")
        .add_string_choice("Closet Cheater", "closet_cheater")
        .add_string_choice("Replays Needed", "replays_needed")
        .add_string_choice("Caution", "caution")
}

fn remove_tag_choices(option: CreateCommandOption<'static>) -> CreateCommandOption<'static> {
    tag_choices(option).add_string_choice("Confirmed Cheater", "confirmed_cheater")
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("tag")
        .description("Manage player tags")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "view",
                "View a player's tags",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "player",
                    "Player name or UUID",
                )
                .required(true),
            ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "add",
                "Add a tag to a player",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "player",
                    "Player name or UUID",
                )
                .required(true),
            )
            .add_sub_option(tag_choices(
                CreateCommandOption::new(CommandOptionType::String, "type", "Tag type")
                    .required(true),
            ))
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::String, "reason", "Reason for the tag")
                    .max_length(120),
            )
            .add_sub_option(CreateCommandOption::new(
                CommandOptionType::Boolean,
                "hide",
                "Hide your username",
            )),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "remove",
                "Remove a tag from a player",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "player",
                    "Player name or UUID",
                )
                .required(true),
            )
            .add_sub_option(remove_tag_choices(
                CreateCommandOption::new(CommandOptionType::String, "type", "Tag type to remove")
                    .required(true),
            )),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "change",
                "Modify an existing tag's reason",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "player",
                    "Player name or UUID",
                )
                .required(true),
            )
            .add_sub_option(tag_choices(
                CreateCommandOption::new(CommandOptionType::String, "type", "Tag type to modify")
                    .required(true),
            ))
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::String, "reason", "New reason")
                    .required(true)
                    .max_length(120),
            ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "lock",
                "Lock a player's tags from modification",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "player",
                    "Player name or UUID",
                )
                .required(true),
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::String, "reason", "Reason for locking")
                    .required(true)
                    .max_length(120),
            ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "unlock",
                "Unlock a player's tags",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "player",
                    "Player name or UUID",
                )
                .required(true),
            ),
        )
}

pub async fn run(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    let subcommand = command.data.options.first().map(|o| o.name.as_str());

    match subcommand {
        Some("view") => run_view(ctx, command, data).await,
        Some("add") => run_add(ctx, command, data).await,
        Some("remove") => run_remove(ctx, command, data).await,
        Some("change") => run_change(ctx, command, data).await,
        Some("lock") => run_lock(ctx, command, data).await,
        Some("unlock") => run_unlock(ctx, command, data).await,
        _ => Ok(()),
    }
}

fn get_sub_options(command: &CommandInteraction) -> Vec<ResolvedOption<'_>> {
    command
        .data
        .options()
        .first()
        .map(|o| match &o.value {
            ResolvedValue::SubCommand(opts) => opts.to_vec(),
            _ => vec![],
        })
        .unwrap_or_default()
}

fn get_string<'a>(options: &'a [ResolvedOption<'a>], name: &str) -> &'a str {
    options
        .iter()
        .find(|o| o.name == name)
        .and_then(|o| match o.value {
            ResolvedValue::String(s) => Some(s),
            _ => None,
        })
        .unwrap_or("")
}

fn get_bool(options: &[ResolvedOption<'_>], name: &str) -> bool {
    options
        .iter()
        .find(|o| o.name == name)
        .and_then(|o| match o.value {
            ResolvedValue::Boolean(b) => Some(b),
            _ => None,
        })
        .unwrap_or(false)
}

fn is_valid_minecraft_name(name: &str) -> bool {
    (3..=16).contains(&name.len())
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub(super) async fn get_rank(data: &Data, discord_id: u64) -> Result<AccessRank> {
    let member_repo = MemberRepository::new(data.db.pool());
    let member = member_repo.get_by_discord_id(discord_id as i64).await?;
    Ok(AccessRank::of(data, discord_id, member.as_ref()))
}

async fn get_rank_and_member(
    data: &Data,
    discord_id: u64,
) -> Result<(AccessRank, Option<database::Member>)> {
    let member_repo = MemberRepository::new(data.db.pool());
    let member = member_repo.get_by_discord_id(discord_id as i64).await?;
    let rank = AccessRank::of(data, discord_id, member.as_ref());
    Ok((rank, member))
}

async fn send_error(ctx: &Context, command: &CommandInteraction, message: &str) -> Result<()> {
    let container = CreateContainer::new(vec![CreateContainerComponent::TextDisplay(
        CreateTextDisplay::new(format!("## Error\n{}", message)),
    )])
    .accent_color(COLOR_DANGER);

    command
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new()
                .flags(MessageFlags::IS_COMPONENTS_V2)
                .components(vec![CreateComponent::Container(container)]),
        )
        .await?;

    Ok(())
}

async fn run_view(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    command.defer(&ctx.http).await?;

    let options = get_sub_options(command);
    let player = get_string(&options, "player");

    let player_info = match data.api.resolve(player).await {
        Ok(info) => info,
        Err(_) => return send_error(ctx, command, "Player not found").await,
    };

    let repo = BlacklistRepository::new(data.db.pool());
    let (player_data, player_tags, face) = tokio::join!(
        repo.get_player(&player_info.uuid),
        repo.get_tags(&player_info.uuid),
        face_attachment(data, &player_info.uuid),
    );
    let player_data = player_data?;
    let player_tags = player_tags?;

    let is_locked = player_data.as_ref().map(|p| p.is_locked).unwrap_or(false);
    let dashed_uuid = format_uuid_dashed(&player_info.uuid);

    if player_tags.is_empty() {
        let header = CreateSection::new(
            vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
                format!(
                    "## No Tags\n`{}` is not tagged.",
                    player_info.username
                ),
            ))],
            CreateSectionAccessory::Thumbnail(face_thumbnail()),
        );

        let uuid_line = CreateTextDisplay::new(format!("-# UUID: {}", dashed_uuid));

        let container = CreateContainer::new(vec![
            CreateContainerComponent::Section(header),
            CreateContainerComponent::TextDisplay(uuid_line),
            CreateContainerComponent::Separator(CreateSeparator::new(true)),
        ]);

        let mut resp = EditInteractionResponse::new()
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(vec![CreateComponent::Container(container)]);
        if let Some(att) = face {
            resp = resp.new_attachment(att);
        }
        command.edit_response(&ctx.http, resp).await?;

        return Ok(());
    }

    let evidence_thread = player_data.as_ref().and_then(|p| p.evidence_thread.as_ref());

    let lock_indicator = if is_locked { " \u{1F512}" } else { "" };

    let title = format!(
        "## {} Tagged User{}\nIGN - `{}`",
        EMOTE_TAG, lock_indicator, player_info.username
    );

    let thumbnail = face_thumbnail();

    let header = CreateSection::new(
        vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
            title,
        ))],
        CreateSectionAccessory::Thumbnail(thumbnail),
    );

    let unique_ids: Vec<i64> = {
        let mut seen = std::collections::HashSet::new();
        let mut ids = Vec::new();
        for tag in &player_tags {
            if !tag.hide_username && seen.insert(tag.added_by) {
                ids.push(tag.added_by);
            }
            if let Some(reviewers) = &tag.reviewed_by {
                for &reviewer_id in reviewers {
                    if seen.insert(reviewer_id) {
                        ids.push(reviewer_id);
                    }
                }
            }
        }
        ids
    };

    let mut join_set = tokio::task::JoinSet::new();
    let http = ctx.http.clone();
    for id in unique_ids {
        let http = http.clone();
        join_set.spawn(async move {
            let name = http
                .get_user(serenity::all::UserId::new(id as u64))
                .await
                .map(|u| u.name.to_string())
                .unwrap_or_else(|_| id.to_string());
            (id, name)
        });
    }

    let mut resolved_names = std::collections::HashMap::new();
    while let Some(Ok((id, name))) = join_set.join_next().await {
        resolved_names.insert(id, name);
    }

    let mut components: Vec<CreateContainerComponent> =
        vec![CreateContainerComponent::Section(header)];

    for tag in &player_tags {
        let def = lookup_tag(&tag.tag_type);
        let emote = def.map(|d| d.emote).unwrap_or("");
        let display_name = def.map(|d| d.display_name).unwrap_or(&tag.tag_type);

        let added_line = if tag.hide_username {
            format!("> -# **\\- <t:{}:R>**", tag.added_on.timestamp())
        } else {
            let fallback = tag.added_by.to_string();
            let username = resolved_names
                .get(&tag.added_by)
                .map(|s| s.as_str())
                .unwrap_or(&fallback);
            format!(
                "> -# **\\- Added by `@{}` <t:{}:R>**",
                username,
                tag.added_on.timestamp()
            )
        };

        let reviewed_line = tag.reviewed_by.as_ref().map(|ids| {
            let names: Vec<String> = ids
                .iter()
                .map(|id| {
                    resolved_names
                        .get(id)
                        .cloned()
                        .unwrap_or_else(|| id.to_string())
                })
                .collect();
            let formatted: Vec<String> = names.iter().map(|n| format!("`@{n}`")).collect();
            format!("> -# **\\- Reviewed by {}**", formatted.join(", "))
        });

        let evidence_indicator = if tag.tag_type == "confirmed_cheater" {
            if evidence_thread.is_some() {
                format!(" {}", EMOTE_EVIDENCE)
            } else {
                format!(" {}", EMOTE_NO_EVIDENCE)
            }
        } else {
            String::new()
        };

        let mut display = format!(
            "{} {}{}\n> {}\n{}",
            emote,
            display_name,
            evidence_indicator,
            sanitize_reason(&tag.reason),
            added_line
        );
        if let Some(line) = reviewed_line {
            display.push('\n');
            display.push_str(&line);
        }

        let tag_display = CreateTextDisplay::new(display);
        components.push(CreateContainerComponent::TextDisplay(tag_display));
    }

    let mut footer = format!("-# UUID: {}", dashed_uuid);
    if let Some(ref evidence_url) = player_data.as_ref().and_then(|p| p.evidence_thread.as_ref())
    {
        footer.push_str(&format!(" | [Evidence]({})", evidence_url));
    }
    components.push(CreateContainerComponent::TextDisplay(
        CreateTextDisplay::new(footer),
    ));
    components.push(CreateContainerComponent::Separator(CreateSeparator::new(
        true,
    )));

    let container = CreateContainer::new(components);

    let mut resp = EditInteractionResponse::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(vec![CreateComponent::Container(container)]);
    if let Some(att) = face {
        resp = resp.new_attachment(att);
    }
    command.edit_response(&ctx.http, resp).await?;

    Ok(())
}

async fn run_add(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    command.defer_ephemeral(&ctx.http).await?;

    let discord_id = command.user.id.get();
    let (rank, member) = get_rank_and_member(data, discord_id).await?;

    let is_linked = member.as_ref().and_then(|m| m.uuid.as_ref()).is_some();

    if !is_linked {
        return send_error(ctx, command, "You must link your account to add tags").await;
    }

    if rank < AccessRank::Helper
        && member.as_ref().map(|m| m.tagging_disabled).unwrap_or(false)
    {
        return send_error(ctx, command, "Your tagging ability has been disabled").await;
    }

    let options = get_sub_options(command);
    let player = get_string(&options, "player");
    let tag_type = get_string(&options, "type");
    let reason = get_string(&options, "reason");
    let hide = get_bool(&options, "hide");

    if tag_type == "confirmed_cheater" {
        return send_error(ctx, command, "Confirmed cheater tags can only be applied through the review system").await;
    }

    if tag_type == "caution" && rank < AccessRank::Moderator {
        return send_error(ctx, command, "Only moderators and above can add caution tags").await;
    }

    if tag_type == "replays_needed" && rank < AccessRank::Member {
        return send_error(ctx, command, "Only members and above can add replays needed tags").await;
    }

    let reason = if tag_type == "replays_needed" { "" } else { reason };

    if reason.is_empty() && tag_type != "replays_needed" {
        return send_error(ctx, command, "A reason is required for this tag type").await;
    }

    let mut needs_review = match rank {
        AccessRank::Default => tag_type != "sniper",
        AccessRank::Member => false,
        _ => false,
    };

    let (player_name, player_uuid, is_nicked) = match data.api.resolve(player).await {
        Ok(info) => (info.username, info.uuid, false),
        Err(_) => (player.to_string(), String::new(), true),
    };

    if is_nicked {
        if !is_valid_minecraft_name(&player_name) {
            return send_error(ctx, command, "Invalid username. Minecraft names can only contain letters, numbers, and underscores (3-16 characters)").await;
        }
        needs_review = true;
    }

    if needs_review {
        let components = super::reviews::build_confirmation_message(
            discord_id,
            &player_name,
            &player_uuid,
            tag_type,
            reason,
            is_nicked,
        );

        command
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(components),
            )
            .await?;

        return Ok(());
    }

    let player_info = crate::api::ResolveResponse {
        username: player_name,
        uuid: player_uuid,
    };

    let repo = BlacklistRepository::new(data.db.pool());

    if let Some(player_data) = repo.get_player(&player_info.uuid).await? {
        if player_data.is_locked {
            return send_error(ctx, command, "This player's tags are locked").await;
        }
    }

    let existing_tags = repo.get_tags(&player_info.uuid).await?;
    let new_priority = lookup_tag(tag_type).map(|d| d.priority).unwrap_or(0);

    let conflicting_tag = existing_tags
        .iter()
        .find(|t| lookup_tag(&t.tag_type).map(|d| d.priority).unwrap_or(0) == new_priority);

    if let Some(conflict) = conflicting_tag {
        if conflict.tag_type == tag_type {
            let components = super::reviews::build_confirmation_message(
                discord_id,
                &player_info.username,
                &player_info.uuid,
                tag_type,
                reason,
                false,
            );

            command
                .edit_response(
                    &ctx.http,
                    EditInteractionResponse::new()
                        .flags(MessageFlags::IS_COMPONENTS_V2)
                        .components(components),
                )
                .await?;

            return Ok(());
        }

        if rank < AccessRank::Member {
            return send_error(ctx, command, "You need member access to overwrite existing tags")
                .await;
        }

        if conflict.tag_type == "confirmed_cheater" && rank < AccessRank::Helper {
            return send_error(
                ctx,
                command,
                "Only helpers and above can overwrite confirmed cheater tags",
            )
            .await;
        }

        let old_def = lookup_tag(&conflict.tag_type);
        let old_emote = old_def.map(|d| d.emote).unwrap_or("");
        let old_display = old_def
            .map(|d| d.display_name)
            .unwrap_or(&conflict.tag_type);

        let new_def = lookup_tag(tag_type);
        let new_emote = new_def.map(|d| d.emote).unwrap_or("");
        let new_display = new_def.map(|d| d.display_name).unwrap_or(tag_type);
        let new_color = new_def.map(|d| d.color).unwrap_or(COLOR_FALLBACK);

        let dashed_uuid = format_uuid_dashed(&player_info.uuid);

        let overwrite_key = command.id.to_string();
        data.pending_overwrites.lock().unwrap().insert(
            overwrite_key.clone(),
            PendingOverwrite {
                uuid: player_info.uuid.clone(),
                old_tag_id: conflict.id,
                tag_type: tag_type.to_string(),
                reason: reason.to_string(),
                hide,
            },
        );

        let button_id = format!("tag_overwrite:{overwrite_key}");

        let button = CreateButton::new(&button_id)
            .label("Overwrite Tag")
            .style(ButtonStyle::Danger);

        let thumbnail = face_thumbnail();

        let header = CreateSection::new(
            vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
                format!(
                    "## {} Tag Overwrite\nIGN - `{}`",
                    EMOTE_EDITTAG, player_info.username
                ),
            ))],
            CreateSectionAccessory::Thumbnail(thumbnail),
        );

        let old_tag_added = format_added_line(ctx, conflict).await;

        let old_tag = CreateTextDisplay::new(format!(
            "{} {}\n> {}\n{}",
            old_emote,
            old_display,
            sanitize_reason(&conflict.reason),
            old_tag_added
        ));

        let new_tag_added = if hide {
            String::new()
        } else {
            format!("\n> -# **\\- Added by `@{}`**", command.user.name)
        };

        let new_tag_section = CreateSection::new(
            vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
                format!(
                    "{} {}\n> {}{}",
                    new_emote,
                    new_display,
                    sanitize_reason(reason),
                    new_tag_added
                ),
            ))],
            CreateSectionAccessory::Button(button),
        );

        let uuid_line = CreateTextDisplay::new(format!("-# UUID: {}", dashed_uuid));

        let container = CreateContainer::new(vec![
            CreateContainerComponent::Section(header),
            CreateContainerComponent::TextDisplay(old_tag),
            CreateContainerComponent::TextDisplay(uuid_line),
            CreateContainerComponent::Separator(CreateSeparator::new(true)),
            CreateContainerComponent::Section(new_tag_section),
        ])
        .accent_color(new_color);

        let mut resp = EditInteractionResponse::new()
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(vec![
                CreateComponent::TextDisplay(CreateTextDisplay::new(
                    "This user already has an incompatible tag! Would you like to overwrite?",
                )),
                CreateComponent::Container(container),
            ]);
        if let Some(att) = face_attachment(data, &player_info.uuid).await {
            resp = resp.new_attachment(att);
        }
        command.edit_response(&ctx.http, resp).await?;

        return Ok(());
    }

    repo.add_tag(&player_info.uuid, tag_type, reason, discord_id as i64, hide, None)
        .await?;

    let player_tags = repo.get_tags(&player_info.uuid).await?;
    let new_tag = player_tags.iter().find(|t| t.tag_type == tag_type);

    let def = lookup_tag(tag_type);
    let emote = def.map(|d| d.emote).unwrap_or("");
    let display_name = def.map(|d| d.display_name).unwrap_or(tag_type);
    let color = def.map(|d| d.color).unwrap_or(COLOR_FALLBACK);
    let dashed_uuid = format_uuid_dashed(&player_info.uuid);

    let thumbnail = face_thumbnail();

    let header = CreateSection::new(
        vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
            format!(
                "## {} New Tag Applied\nIGN - `{}`",
                EMOTE_ADDTAG, player_info.username
            ),
        ))],
        CreateSectionAccessory::Thumbnail(thumbnail),
    );

    let added_line = match &new_tag {
        Some(tag) => format_added_line(ctx, tag).await,
        None if hide => String::new(),
        None => format!("\n> -# **\\- Added by `@{}`**", command.user.name),
    };

    let tag_display = CreateTextDisplay::new(format!(
        "{} {}\n> {}\n{}",
        emote,
        display_name,
        sanitize_reason(reason),
        added_line
    ));

    let uuid_line = CreateTextDisplay::new(format!("-# UUID: {}", dashed_uuid));

    let channel_msg_id = if let Some(tag) = &new_tag {
        post_new_tag(ctx, data, &player_info.uuid, &player_info.username, tag)
            .await
            .map(|id| id.get())
            .unwrap_or(0)
    } else {
        0
    };

    let tag_id = new_tag.map(|t| t.id).unwrap_or(0);
    let undo_id = format!("tag_undo:{tag_id}:{channel_msg_id}");
    let edit_id = format!("tag_edit:{tag_id}:{channel_msg_id}");

    let container = CreateContainer::new(vec![
        CreateContainerComponent::Section(header),
        CreateContainerComponent::TextDisplay(tag_display),
        CreateContainerComponent::TextDisplay(uuid_line),
        CreateContainerComponent::Separator(CreateSeparator::new(true)),
        CreateContainerComponent::ActionRow(CreateActionRow::buttons(vec![
            CreateButton::new(edit_id)
                .label("Edit")
                .style(ButtonStyle::Secondary),
            CreateButton::new(undo_id)
                .label("Undo")
                .style(ButtonStyle::Danger),
        ])),
        CreateContainerComponent::TextDisplay(CreateTextDisplay::new(
            "-# You can also use /tag change within 30 minutes to update this tag",
        )),
    ])
    .accent_color(color);

    let mut resp = EditInteractionResponse::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(vec![CreateComponent::Container(container)]);
    if let Some(att) = face_attachment(data, &player_info.uuid).await {
        resp = resp.new_attachment(att);
    }
    command.edit_response(&ctx.http, resp).await?;

    Ok(())
}

pub async fn handle_overwrite_button(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let key = component
        .data
        .custom_id
        .strip_prefix("tag_overwrite:")
        .unwrap_or_default();

    let overwrite = data.pending_overwrites.lock().unwrap().remove(key);
    let Some(overwrite) = overwrite else {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("This overwrite has expired")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    };

    let uuid = &overwrite.uuid;
    let old_tag_id = overwrite.old_tag_id;
    let new_tag_type = overwrite.tag_type.as_str();
    let hide = overwrite.hide;
    let reason = &overwrite.reason;

    let cache = CacheRepository::new(data.db.pool());
    let player_name = cache
        .get_username(uuid)
        .await
        .ok()
        .flatten()
        .unwrap_or_else(|| uuid.to_string());

    let discord_id = component.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    if rank < AccessRank::Member {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("You need member access to overwrite tags")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let repo = BlacklistRepository::new(data.db.pool());

    if let Some(player_data) = repo.get_player(uuid).await? {
        if player_data.is_locked {
            component
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("This player's tags are locked")
                            .ephemeral(true),
                    ),
                )
                .await?;
            return Ok(());
        }
    }

    let existing_tags = repo.get_tags(uuid).await?;
    let old_tag = existing_tags.iter().find(|t| t.id == old_tag_id);

    let Some(old_tag) = old_tag else {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("The original tag no longer exists")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    };

    if old_tag.tag_type == "confirmed_cheater" && rank < AccessRank::Helper {
        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Only helpers and above can overwrite confirmed cheater tags")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    let old_tag_clone = old_tag.clone();

    repo.remove_tag(old_tag_id, discord_id as i64).await?;
    repo.add_tag(uuid, new_tag_type, &reason, discord_id as i64, hide, None)
        .await?;

    let new_tags = repo.get_tags(uuid).await?;
    let new_tag = new_tags.iter().find(|t| t.tag_type == new_tag_type);

    let def = lookup_tag(new_tag_type);
    let emote = def.map(|d| d.emote).unwrap_or("");
    let display_name = def.map(|d| d.display_name).unwrap_or(new_tag_type);
    let color = def.map(|d| d.color).unwrap_or(COLOR_FALLBACK);
    let dashed_uuid = format_uuid_dashed(uuid);

    let thumbnail = face_thumbnail();

    let header = CreateSection::new(
        vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
            format!(
                "## {} Tag Overwritten\nIGN - `{}`",
                EMOTE_EDITTAG, player_name
            ),
        ))],
        CreateSectionAccessory::Thumbnail(thumbnail),
    );

    let added_line = match &new_tag {
        Some(tag) => format_added_line(ctx, tag).await,
        None if hide => String::new(),
        None => format!("\n> -# **\\- Added by `@{}`**", component.user.name),
    };

    let tag_display = CreateTextDisplay::new(format!(
        "{} {}\n> {}\n{}",
        emote,
        display_name,
        sanitize_reason(&reason),
        added_line
    ));

    let uuid_line = CreateTextDisplay::new(format!("-# UUID: {}", dashed_uuid));

    let container = CreateContainer::new(vec![
        CreateContainerComponent::Section(header),
        CreateContainerComponent::TextDisplay(tag_display),
        CreateContainerComponent::TextDisplay(uuid_line),
        CreateContainerComponent::Separator(CreateSeparator::new(true)),
    ])
    .accent_color(color);

    let mut msg = CreateInteractionResponseMessage::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(vec![CreateComponent::Container(container)]);
    if let Some(att) = face_attachment(data, uuid).await {
        msg = msg.add_file(att);
    }
    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(msg),
        )
        .await?;

    if let Some(new_tag) = &new_tag {
        post_tag_changed(
            ctx,
            data,
            uuid,
            &player_name,
            &old_tag_clone,
            new_tag,
            "Tag Overwritten",
            discord_id,
        )
        .await;
        post_overwritten_tag(ctx, data, uuid, &player_name, new_tag).await;
    }

    Ok(())
}

async fn run_remove(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    command.defer_ephemeral(&ctx.http).await?;

    let discord_id = command.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    if rank < AccessRank::Helper {
        return send_error(ctx, command, "Only helpers and above can remove tags").await;
    }

    let options = get_sub_options(command);
    let player = get_string(&options, "player");
    let tag_type = get_string(&options, "type");

    if (tag_type == "confirmed_cheater" || tag_type == "caution") && rank < AccessRank::Moderator {
        return send_error(ctx, command, "Only moderators and above can remove this tag type").await;
    }

    let player_info = match data.api.resolve(player).await {
        Ok(info) => info,
        Err(_) => return send_error(ctx, command, "Player not found").await,
    };

    let repo = BlacklistRepository::new(data.db.pool());

    if let Some(player_data) = repo.get_player(&player_info.uuid).await? {
        if player_data.is_locked {
            return send_error(ctx, command, "This player's tags are locked").await;
        }
    }

    let player_tags = repo.get_tags(&player_info.uuid).await?;
    let tag = player_tags.iter().find(|t| t.tag_type == tag_type);

    let Some(tag) = tag else {
        return send_error(
            ctx,
            command,
            &format!("Player doesn't have a {} tag", tag_type),
        )
        .await;
    };

    let tag_clone = tag.clone();
    let removed = repo.remove_tag(tag.id, discord_id as i64).await?;

    if !removed {
        return send_error(ctx, command, "Failed to remove tag").await;
    }

    if tag_type == "confirmed_cheater" {
        if let Some(player_data) = repo.get_player(&player_info.uuid).await? {
            if let Some(thread_url) = &player_data.evidence_thread {
                super::evidence::archive_evidence_by_url(ctx, data, thread_url).await?;
            }
        }
    }

    let def = lookup_tag(tag_type);
    let emote = def.map(|d| d.emote).unwrap_or("");
    let display_name = def.map(|d| d.display_name).unwrap_or(tag_type);
    let dashed_uuid = format_uuid_dashed(&player_info.uuid);

    let thumbnail = face_thumbnail();

    let header = CreateSection::new(
        vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
            format!(
                "## {} Tag Removed\nIGN - `{}`",
                EMOTE_REMOVETAG, player_info.username
            ),
        ))],
        CreateSectionAccessory::Thumbnail(thumbnail),
    );

    let added_line = format_added_line(ctx, &tag_clone).await;
    let tag_display = CreateTextDisplay::new(format!(
        "{} {}\n> {}\n{}",
        emote,
        display_name,
        sanitize_reason(&tag_clone.reason),
        added_line
    ));

    let uuid_line = CreateTextDisplay::new(format!("-# UUID: {}", dashed_uuid));

    let container = CreateContainer::new(vec![
        CreateContainerComponent::Section(header),
        CreateContainerComponent::TextDisplay(tag_display),
        CreateContainerComponent::TextDisplay(uuid_line),
        CreateContainerComponent::Separator(CreateSeparator::new(true)),
    ])
    .accent_color(COLOR_DANGER);

    let mut resp = EditInteractionResponse::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(vec![CreateComponent::Container(container)]);
    if let Some(att) = face_attachment(data, &player_info.uuid).await {
        resp = resp.new_attachment(att);
    }
    command.edit_response(&ctx.http, resp).await?;

    post_tag_removed(
        ctx,
        data,
        &player_info.uuid,
        &player_info.username,
        &tag_clone,
        discord_id,
    )
    .await;

    Ok(())
}

async fn run_change(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    command.defer_ephemeral(&ctx.http).await?;

    let discord_id = command.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    if rank < AccessRank::Member {
        return send_error(ctx, command, "You need member access to modify tags").await;
    }

    let options = get_sub_options(command);
    let player = get_string(&options, "player");
    let tag_type = get_string(&options, "type");
    let new_reason = get_string(&options, "reason");

    let player_info = match data.api.resolve(player).await {
        Ok(info) => info,
        Err(_) => return send_error(ctx, command, "Player not found").await,
    };

    let repo = BlacklistRepository::new(data.db.pool());

    if let Some(player_data) = repo.get_player(&player_info.uuid).await? {
        if player_data.is_locked {
            return send_error(ctx, command, "This player's tags are locked").await;
        }
    }

    let player_tags = repo.get_tags(&player_info.uuid).await?;
    let tag = player_tags.iter().find(|t| t.tag_type == tag_type);

    let Some(tag) = tag else {
        return send_error(
            ctx,
            command,
            &format!("Player doesn't have a {} tag", tag_type),
        )
        .await;
    };

    let is_own_tag = tag.added_by == discord_id as i64;

    if !is_own_tag && rank < AccessRank::Helper {
        return send_error(ctx, command, "You can only modify your own tags").await;
    }

    if is_own_tag && rank < AccessRank::Helper {
        let age = chrono::Utc::now().signed_duration_since(tag.added_on);
        if age.num_minutes() > 30 {
            return send_error(
                ctx,
                command,
                "The 30-minute edit window has passed. Use a tag review to request changes.",
            )
            .await;
        }
    }

    let old_tag = tag.clone();
    let modified = repo.modify_tag(tag.id, None, Some(new_reason)).await?;

    if !modified {
        return send_error(ctx, command, "Failed to modify tag").await;
    }

    let def = lookup_tag(tag_type);
    let emote = def.map(|d| d.emote).unwrap_or("");
    let display_name = def.map(|d| d.display_name).unwrap_or(tag_type);
    let color = def.map(|d| d.color).unwrap_or(COLOR_FALLBACK);
    let dashed_uuid = format_uuid_dashed(&player_info.uuid);

    let thumbnail = face_thumbnail();

    let header = CreateSection::new(
        vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
            format!(
                "## {} Tag Modified\nIGN - `{}`",
                EMOTE_EDITTAG, player_info.username
            ),
        ))],
        CreateSectionAccessory::Thumbnail(thumbnail),
    );

    let added_line = format_added_line(ctx, tag).await;
    let tag_display = CreateTextDisplay::new(format!(
        "{} {}\n> {} **->** {}\n{}",
        emote,
        display_name,
        sanitize_reason(&old_tag.reason),
        sanitize_reason(new_reason),
        added_line
    ));

    let uuid_line = CreateTextDisplay::new(format!("-# UUID: {}", dashed_uuid));

    let container = CreateContainer::new(vec![
        CreateContainerComponent::Section(header),
        CreateContainerComponent::TextDisplay(tag_display),
        CreateContainerComponent::TextDisplay(uuid_line),
        CreateContainerComponent::Separator(CreateSeparator::new(true)),
    ])
    .accent_color(color);

    let mut resp = EditInteractionResponse::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(vec![CreateComponent::Container(container)]);
    if let Some(att) = face_attachment(data, &player_info.uuid).await {
        resp = resp.new_attachment(att);
    }
    command.edit_response(&ctx.http, resp).await?;

    let updated_tags = repo.get_tags(&player_info.uuid).await?;
    if let Some(new_tag) = updated_tags.iter().find(|t| t.tag_type == tag_type) {
        post_tag_changed(
            ctx,
            data,
            &player_info.uuid,
            &player_info.username,
            &old_tag,
            new_tag,
            "Tag Modified",
            discord_id,
        )
        .await;
    }

    Ok(())
}

async fn run_lock(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    command.defer_ephemeral(&ctx.http).await?;

    let discord_id = command.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    if rank < AccessRank::Moderator {
        return send_error(ctx, command, "Only moderators can lock players").await;
    }

    let options = get_sub_options(command);
    let player = get_string(&options, "player");
    let reason = get_string(&options, "reason");

    let player_info = match data.api.resolve(player).await {
        Ok(info) => info,
        Err(_) => return send_error(ctx, command, "Player not found").await,
    };

    let repo = BlacklistRepository::new(data.db.pool());
    repo.lock_player(&player_info.uuid, reason, discord_id as i64)
        .await?;

    let dashed_uuid = format_uuid_dashed(&player_info.uuid);

    let thumbnail = face_thumbnail();

    let header = CreateSection::new(
        vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
            format!(
                "## {} Player Locked 🔒\nIGN - `{}`",
                EMOTE_TAG, player_info.username
            ),
        ))],
        CreateSectionAccessory::Thumbnail(thumbnail),
    );

    let reason_display = CreateTextDisplay::new(format!("> {}", sanitize_reason(reason)));
    let uuid_line = CreateTextDisplay::new(format!("-# UUID: {}", dashed_uuid));

    let container = CreateContainer::new(vec![
        CreateContainerComponent::Section(header),
        CreateContainerComponent::TextDisplay(reason_display),
        CreateContainerComponent::TextDisplay(uuid_line),
        CreateContainerComponent::Separator(CreateSeparator::new(true)),
    ])
    .accent_color(COLOR_DANGER);

    let mut resp = EditInteractionResponse::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(vec![CreateComponent::Container(container)]);
    if let Some(att) = face_attachment(data, &player_info.uuid).await {
        resp = resp.new_attachment(att);
    }
    command.edit_response(&ctx.http, resp).await?;

    post_lock_change(
        ctx,
        data,
        &player_info.uuid,
        &player_info.username,
        true,
        Some(reason),
        discord_id,
    )
    .await;

    Ok(())
}

async fn run_unlock(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    command.defer_ephemeral(&ctx.http).await?;

    let discord_id = command.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    if rank < AccessRank::Moderator {
        return send_error(ctx, command, "Only moderators can unlock players").await;
    }

    let options = get_sub_options(command);
    let player = get_string(&options, "player");

    let player_info = match data.api.resolve(player).await {
        Ok(info) => info,
        Err(_) => return send_error(ctx, command, "Player not found").await,
    };

    let repo = BlacklistRepository::new(data.db.pool());
    let unlocked = repo.unlock_player(&player_info.uuid).await?;

    let dashed_uuid = format_uuid_dashed(&player_info.uuid);
    let face = face_attachment(data, &player_info.uuid).await;

    if !unlocked {
        let header = CreateSection::new(
            vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
                format!("## Not Locked\nIGN - `{}`", player_info.username),
            ))],
            CreateSectionAccessory::Thumbnail(face_thumbnail()),
        );

        let uuid_line = CreateTextDisplay::new(format!("-# UUID: {}", dashed_uuid));

        let container = CreateContainer::new(vec![
            CreateContainerComponent::Section(header),
            CreateContainerComponent::TextDisplay(uuid_line),
            CreateContainerComponent::Separator(CreateSeparator::new(true)),
        ]);

        let mut resp = EditInteractionResponse::new()
            .flags(MessageFlags::IS_COMPONENTS_V2)
            .components(vec![CreateComponent::Container(container)]);
        if let Some(att) = face {
            resp = resp.new_attachment(att);
        }
        command.edit_response(&ctx.http, resp).await?;

        return Ok(());
    }

    let header = CreateSection::new(
        vec![CreateSectionComponent::TextDisplay(CreateTextDisplay::new(
            format!(
                "## {} Player Unlocked 🔓\nIGN - `{}`",
                EMOTE_TAG, player_info.username
            ),
        ))],
        CreateSectionAccessory::Thumbnail(face_thumbnail()),
    );

    let uuid_line = CreateTextDisplay::new(format!("-# UUID: {}", dashed_uuid));

    let container = CreateContainer::new(vec![
        CreateContainerComponent::Section(header),
        CreateContainerComponent::TextDisplay(uuid_line),
        CreateContainerComponent::Separator(CreateSeparator::new(true)),
    ])
    .accent_color(COLOR_SUCCESS);

    let mut resp = EditInteractionResponse::new()
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(vec![CreateComponent::Container(container)]);
    if let Some(att) = face {
        resp = resp.new_attachment(att);
    }
    command.edit_response(&ctx.http, resp).await?;

    post_lock_change(
        ctx,
        data,
        &player_info.uuid,
        &player_info.username,
        false,
        None,
        discord_id,
    )
    .await;

    Ok(())
}

pub async fn handle_undo(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let parts: Vec<&str> = component.data.custom_id.splitn(3, ':').collect();
    if parts.len() < 3 {
        return Ok(());
    }

    let tag_id: i64 = parts[1].parse().unwrap_or(0);
    let channel_msg_id: u64 = parts[2].parse().unwrap_or(0);

    let discord_id = component.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    let repo = BlacklistRepository::new(data.db.pool());
    let Some(tag) = repo.get_tag_by_id(tag_id).await? else {
        return send_component_message(ctx, component, "Tag not found or already removed").await;
    };

    if tag.added_by != discord_id as i64 && rank < AccessRank::Helper {
        return send_component_message(ctx, component, "You can only undo your own tags").await;
    }

    if rank < AccessRank::Helper {
        let age = chrono::Utc::now().signed_duration_since(tag.added_on);
        if age.num_minutes() > 30 {
            return send_component_message(ctx, component, "The 30-minute undo window has passed")
                .await;
        }
    }

    repo.remove_tag(tag_id, discord_id as i64).await?;

    if channel_msg_id != 0 {
        if let Some(channel_id) = data.blacklist_channel_id {
            let _ = ctx
                .http
                .delete_message(
                    channel_id.into(),
                    serenity::all::MessageId::new(channel_msg_id),
                    None,
                )
                .await;
        }
    }

    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(vec![CreateComponent::Container(
                        CreateContainer::new(vec![CreateContainerComponent::TextDisplay(
                            CreateTextDisplay::new(format!("## {} Tag Removed", EMOTE_REMOVETAG)),
                        )])
                        .accent_color(COLOR_DANGER),
                    )]),
            ),
        )
        .await?;

    Ok(())
}

pub async fn handle_edit(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let parts: Vec<&str> = component.data.custom_id.splitn(3, ':').collect();
    if parts.len() < 3 {
        return Ok(());
    }

    let tag_id: i64 = parts[1].parse().unwrap_or(0);
    let channel_msg_id: u64 = parts[2].parse().unwrap_or(0);

    let discord_id = component.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    let repo = BlacklistRepository::new(data.db.pool());
    let Some(tag) = repo.get_tag_by_id(tag_id).await? else {
        return send_component_message(ctx, component, "Tag not found or already removed").await;
    };

    if tag.added_by != discord_id as i64 && rank < AccessRank::Helper {
        return send_component_message(ctx, component, "You can only edit your own tags").await;
    }

    if rank < AccessRank::Helper {
        let age = chrono::Utc::now().signed_duration_since(tag.added_on);
        if age.num_minutes() > 30 {
            return send_component_message(ctx, component, "The 30-minute edit window has passed")
                .await;
        }
    }

    let select = serenity::all::CreateSelectMenu::new(
        format!("tag_edit_type:{tag_id}:{channel_msg_id}"),
        serenity::all::CreateSelectMenuKind::String {
            options: tag_choices_for_edit(&tag.tag_type).into(),
        },
    )
    .placeholder("Change tag type");

    let reason_btn = CreateButton::new(format!("tag_edit_reason:{tag_id}:{channel_msg_id}"))
        .label("Change Reason")
        .style(ButtonStyle::Secondary);

    let remove_btn = CreateButton::new(format!("tag_undo:{tag_id}:{channel_msg_id}"))
        .label("Remove")
        .style(ButtonStyle::Danger);

    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(vec![
                        CreateComponent::Container(
                            CreateContainer::new(vec![
                                CreateContainerComponent::TextDisplay(CreateTextDisplay::new(
                                    format!("## {} Edit Tag", EMOTE_EDITTAG),
                                )),
                                CreateContainerComponent::ActionRow(
                                    CreateActionRow::SelectMenu(select),
                                ),
                                CreateContainerComponent::ActionRow(CreateActionRow::buttons(
                                    vec![reason_btn, remove_btn],
                                )),
                            ])
                            .accent_color(COLOR_INFO),
                        ),
                    ]),
            ),
        )
        .await?;

    Ok(())
}

pub async fn handle_edit_type(
    ctx: &Context,
    component: &ComponentInteraction,
    data: &Data,
) -> Result<()> {
    let parts: Vec<&str> = component.data.custom_id.splitn(3, ':').collect();
    if parts.len() < 3 {
        return Ok(());
    }

    let tag_id: i64 = parts[1].parse().unwrap_or(0);
    let channel_msg_id: u64 = parts[2].parse().unwrap_or(0);

    let new_type = match &component.data.kind {
        serenity::all::ComponentInteractionDataKind::StringSelect { values } => {
            values.first().map(|s| s.as_str()).unwrap_or("")
        }
        _ => return Ok(()),
    };

    let discord_id = component.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    let repo = BlacklistRepository::new(data.db.pool());
    let Some(tag) = repo.get_tag_by_id(tag_id).await? else {
        return send_component_message(ctx, component, "Tag not found or already removed").await;
    };

    if tag.added_by != discord_id as i64 && rank < AccessRank::Helper {
        return send_component_message(ctx, component, "Insufficient permissions").await;
    }

    if new_type == "confirmed_cheater" {
        return send_component_message(ctx, component, "Confirmed cheater tags can only be applied through the review system").await;
    }

    if new_type == "caution" && rank < AccessRank::Moderator {
        return send_component_message(ctx, component, "Only moderators and above can assign caution tags").await;
    }

    if new_type == "__revert" {
        if rank < AccessRank::Moderator {
            return send_component_message(ctx, component, "Only moderators and above can revert confirmed cheater tags").await;
        }

        let uuid = repo
            .get_uuid_by_player_id(tag.player_id)
            .await
            .ok()
            .flatten()
            .unwrap_or_default();

        if !uuid.is_empty() {
            if let Some(player_data) = repo.get_player(&uuid).await? {
                if let Some(thread_url) = &player_data.evidence_thread {
                    super::evidence::archive_evidence_by_url(ctx, data, thread_url).await?;
                }
            }
        }

        let reverted_tag = repo.get_tag_by_id(tag_id).await?.unwrap_or(tag);
        let reverted_def = lookup_tag(&reverted_tag.tag_type);
        let reverted_display = reverted_def.map(|d| d.display_name).unwrap_or(&reverted_tag.tag_type);

        component
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .flags(MessageFlags::IS_COMPONENTS_V2)
                        .components(vec![CreateComponent::Container(
                            CreateContainer::new(vec![CreateContainerComponent::TextDisplay(
                                CreateTextDisplay::new(format!(
                                    "## {} Tag Reverted\nReverted to **{}**",
                                    EMOTE_EDITTAG, reverted_display
                                )),
                            )])
                            .accent_color(COLOR_SUCCESS),
                        )]),
                ),
            )
            .await?;

        return Ok(());
    }

    let old_tag = tag.clone();
    repo.modify_tag(tag_id, Some(new_type), None).await?;

    if channel_msg_id != 0 {
        if let Some(channel_id) = data.blacklist_channel_id {
            let _ = ctx
                .http
                .delete_message(
                    channel_id.into(),
                    serenity::all::MessageId::new(channel_msg_id),
                    None,
                )
                .await;
        }

        let uuid = repo
            .get_uuid_by_player_id(old_tag.player_id)
            .await
            .ok()
            .flatten()
            .unwrap_or_default();

        if !uuid.is_empty() {
            let updated_tags = repo.get_tags(&uuid).await?;
            if let Some(new_tag) = updated_tags.iter().find(|t| t.id == tag_id) {
                let cache = CacheRepository::new(data.db.pool());
                let name = cache
                    .get_username(&uuid)
                    .await
                    .ok()
                    .flatten()
                    .unwrap_or_else(|| uuid.clone());
                post_new_tag(ctx, data, &uuid, &name, new_tag).await;
            }
        }
    }

    let def = lookup_tag(new_type);
    let display = def.map(|d| d.display_name).unwrap_or(new_type);

    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(vec![CreateComponent::Container(
                        CreateContainer::new(vec![CreateContainerComponent::TextDisplay(
                            CreateTextDisplay::new(format!(
                                "## {} Tag Updated\nType changed to **{}**",
                                EMOTE_EDITTAG, display
                            )),
                        )])
                        .accent_color(COLOR_SUCCESS),
                    )]),
            ),
        )
        .await?;

    Ok(())
}

pub async fn handle_edit_reason(
    ctx: &Context,
    component: &ComponentInteraction,
    _data: &Data,
) -> Result<()> {
    let parts: Vec<&str> = component.data.custom_id.splitn(3, ':').collect();
    if parts.len() < 3 {
        return Ok(());
    }

    let tag_id = parts[1];
    let channel_msg_id = parts[2];

    let input = CreateInputText::new(InputTextStyle::Paragraph, "tag_reason")
        .placeholder("New reason for this tag")
        .required(true);
    let label = CreateLabel::input_text("Reason", input);
    let modal = CreateModal::new(
        format!("tag_edit_reason_modal:{tag_id}:{channel_msg_id}"),
        "Edit Tag Reason",
    )
    .components(vec![CreateModalComponent::Label(label)]);

    component
        .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
        .await?;

    Ok(())
}

pub async fn handle_edit_reason_modal(
    ctx: &Context,
    modal: &serenity::all::ModalInteraction,
    data: &Data,
) -> Result<()> {
    let parts: Vec<&str> = modal.data.custom_id.splitn(3, ':').collect();
    if parts.len() < 3 {
        return Ok(());
    }

    let tag_id: i64 = parts[1].parse().unwrap_or(0);
    let channel_msg_id: u64 = parts[2].parse().unwrap_or(0);

    let new_reason =
        crate::interact::extract_modal_value(&modal.data.components, "tag_reason");

    let discord_id = modal.user.id.get();
    let rank = get_rank(data, discord_id).await?;

    let repo = BlacklistRepository::new(data.db.pool());
    let Some(tag) = repo.get_tag_by_id(tag_id).await? else {
        modal
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Tag not found or already removed")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    };

    if tag.added_by != discord_id as i64 && rank < AccessRank::Helper {
        modal
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Insufficient permissions")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    repo.modify_tag(tag_id, None, Some(&new_reason)).await?;

    if channel_msg_id != 0 {
        if let Some(channel_id) = data.blacklist_channel_id {
            let _ = ctx
                .http
                .delete_message(
                    channel_id.into(),
                    serenity::all::MessageId::new(channel_msg_id),
                    None,
                )
                .await;
        }

        let uuid = repo
            .get_uuid_by_player_id(tag.player_id)
            .await
            .ok()
            .flatten()
            .unwrap_or_default();

        if !uuid.is_empty() {
            let updated_tags = repo.get_tags(&uuid).await?;
            if let Some(new_tag) = updated_tags.iter().find(|t| t.id == tag_id) {
                let cache = CacheRepository::new(data.db.pool());
                let name = cache
                    .get_username(&uuid)
                    .await
                    .ok()
                    .flatten()
                    .unwrap_or_else(|| uuid.clone());
                post_new_tag(ctx, data, &uuid, &name, new_tag).await;
            }
        }
    }

    modal
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(vec![CreateComponent::Container(
                        CreateContainer::new(vec![CreateContainerComponent::TextDisplay(
                            CreateTextDisplay::new(format!(
                                "## {} Reason Updated",
                                EMOTE_EDITTAG
                            )),
                        )])
                        .accent_color(COLOR_SUCCESS),
                    )])
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}

fn tag_choices_for_edit(current: &str) -> Vec<CreateSelectMenuOption<'static>> {
    let choices = [
        ("Sniper", "sniper"),
        ("Blatant Cheater", "blatant_cheater"),
        ("Closet Cheater", "closet_cheater"),
        ("Replays Needed", "replays_needed"),
        ("Caution", "caution"),
    ];

    let mut options: Vec<CreateSelectMenuOption<'static>> = choices
        .into_iter()
        .filter(|(_, value)| *value != current)
        .map(|(label, value)| CreateSelectMenuOption::new(label, value))
        .collect();

    if current == "confirmed_cheater" {
        options.insert(
            0,
            CreateSelectMenuOption::new("Revert to Unconfirmed", "__revert"),
        );
    }

    options
}

async fn send_component_message(
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
