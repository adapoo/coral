pub mod bedwars;
pub mod prestiges;
pub mod session;

use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::Result;
use hypixel::{BedwarsPlayerStats, GuildInfo, Mode};
use image::RgbaImage;
use serenity::all::{
    ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, EditInteractionResponse, Http,
    MessageFlags,
};

use database::CacheRepository;
use render::TagIcon;

use crate::api::{GuildResponse, TagInfo};
use crate::framework::Data;

pub(super) const CACHE_TTL_SECS: u64 = 2 * 60;

const MODE_CHOICES: &[(Mode, &str)] = &[
    (Mode::Overall, "overall"),
    (Mode::Core, "core"),
    (Mode::Solos, "solos"),
    (Mode::Doubles, "doubles"),
    (Mode::Threes, "threes"),
    (Mode::Fours, "fours"),
];

pub fn create_mode_dropdown(
    custom_id: &str,
    cache_key: &str,
    current: Mode,
    stats: &BedwarsPlayerStats,
) -> CreateSelectMenu<'static> {
    let options: Vec<CreateSelectMenuOption<'static>> = MODE_CHOICES
        .iter()
        .map(|(mode, value)| {
            let mode_stats = stats.get_mode_stats(*mode);
            CreateSelectMenuOption::new(mode.display_name(), format!("{value}:{cache_key}"))
                .default_selection(*mode == current)
                .description(format!(
                    "{:.2} fkdr, {:.2} wlr",
                    mode_stats.fkdr(),
                    mode_stats.wlr()
                ))
        })
        .collect();

    CreateSelectMenu::new(
        custom_id.to_string(),
        CreateSelectMenuKind::String {
            options: options.into(),
        },
    )
    .placeholder(current.display_name())
}

pub fn parse_mode_value(value: &str) -> Option<(&str, Mode)> {
    let (mode_str, cache_key) = value.split_once(':')?;
    let mode = Mode::from_str(mode_str)?;
    Some((cache_key, mode))
}

pub fn extract_select_value(component: &ComponentInteraction) -> Option<&str> {
    match &component.data.kind {
        serenity::all::ComponentInteractionDataKind::StringSelect { values } => {
            values.first().map(|s| s.as_str())
        }
        _ => None,
    }
}

pub fn encode_png(image: &RgbaImage) -> Result<Vec<u8>> {
    let mut buf = Cursor::new(Vec::new());
    image.write_to(&mut buf, image::ImageFormat::Png)?;
    Ok(buf.into_inner())
}

pub fn extract_tag_icons(tags: &[TagInfo]) -> Vec<TagIcon> {
    tags.iter()
        .filter_map(|t| blacklist::lookup(&t.tag_type))
        .map(|def| (def.icon.to_string(), def.color))
        .collect()
}

pub(crate) fn looks_like_uuid(s: &str) -> bool {
    let s = s.replace('-', "");
    s.len() == 32 && s.chars().all(|c| c.is_ascii_hexdigit())
}

pub(crate) fn to_guild_info(guild: &GuildResponse) -> GuildInfo {
    let player = guild.player.as_ref();

    let joined = player
        .and_then(|p| p.joined.as_ref())
        .and_then(|j| chrono::DateTime::parse_from_rfc3339(j).ok())
        .map(|dt| dt.timestamp_millis());

    GuildInfo {
        name: Some(guild.name.clone()),
        tag: guild.tag.clone(),
        tag_color: guild.tag_color.clone(),
        rank: player.and_then(|p| p.rank.clone()),
        joined,
        weekly_gexp: player.and_then(|p| p.weekly_gexp),
    }
}

pub use crate::interact::send_deferred_error;

pub async fn disable_components(ctx: &Context, component: &ComponentInteraction) -> Result<()> {
    let gallery = extract_gallery_components(&component.message.components);
    component
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(gallery),
            ),
        )
        .await?;
    Ok(())
}

fn extract_gallery_components(
    components: &[serenity::all::Component],
) -> Vec<serenity::all::CreateComponent<'static>> {
    components
        .iter()
        .filter_map(|c| match c {
            serenity::all::Component::MediaGallery(g) => {
                let items: Vec<_> = g
                    .items
                    .iter()
                    .map(|item| {
                        let url = item
                            .media
                            .proxy_url
                            .as_ref()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| item.media.url.to_string());
                        serenity::all::CreateMediaGalleryItem::new(
                            serenity::all::CreateUnfurledMediaItem::new(url),
                        )
                    })
                    .collect();
                Some(serenity::all::CreateComponent::MediaGallery(
                    serenity::all::CreateMediaGallery::new(items),
                ))
            }
            _ => None,
        })
        .collect()
}

pub(super) async fn resolve_uuid(data: &Data, player: &str) -> Option<String> {
    if looks_like_uuid(player) {
        Some(player.replace('-', "").to_lowercase())
    } else {
        CacheRepository::new(data.db.pool())
            .resolve_uuid(player)
            .await
            .ok()
            .flatten()
    }
}

pub(super) fn spawn_expiry<T: Send + 'static>(
    http: Arc<Http>,
    token: String,
    cache: Arc<Mutex<std::collections::HashMap<String, T>>>,
    cache_key: String,
    get_last_interaction: fn(&T) -> Instant,
) {
    spawn_expiry_with_retain(http, token, cache, cache_key, get_last_interaction, vec![]);
}

pub(super) fn spawn_expiry_with_retain<T: Send + 'static>(
    http: Arc<Http>,
    token: String,
    cache: Arc<Mutex<std::collections::HashMap<String, T>>>,
    cache_key: String,
    get_last_interaction: fn(&T) -> Instant,
    retain: Vec<serenity::all::CreateComponent<'static>>,
) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(CACHE_TTL_SECS)).await;

            let remaining = {
                let store = cache.lock().unwrap();
                match store.get(&cache_key) {
                    Some(entry) => {
                        let elapsed = get_last_interaction(entry).elapsed().as_secs();
                        if elapsed >= CACHE_TTL_SECS {
                            None
                        } else {
                            Some(CACHE_TTL_SECS - elapsed)
                        }
                    }
                    None => None,
                }
            };

            match remaining {
                Some(secs) => tokio::time::sleep(Duration::from_secs(secs)).await,
                None => {
                    cache.lock().unwrap().remove(&cache_key);
                    let mut edit = EditInteractionResponse::new().components(retain.clone());
                    if !retain.is_empty() {
                        edit = edit.flags(MessageFlags::IS_COMPONENTS_V2);
                    }
                    let _ = edit.execute(&http, &token).await;
                    break;
                }
            }
        }
    });
}

pub(super) async fn fetch_skin(
    data: &Data,
    uuid: &str,
    skin_url: Option<&str>,
) -> Option<clients::SkinImage> {
    match skin_url {
        Some(url) => data.skin_provider.fetch_with_url(uuid, url).await,
        None => data.skin_provider.fetch(uuid).await,
    }
}
