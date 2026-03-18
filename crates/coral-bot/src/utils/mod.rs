use serenity::all::{CreateContainerComponent, CreateSeparator, CreateTextDisplay};

use database::CacheRepository;

use crate::framework::Data;

mod minecraft;

pub use minecraft::{format_number, format_uuid_dashed, generate_api_key, sanitize_reason};

pub async fn resolve_username(uuid: &str, data: &Data) -> Option<String> {
    let cache = CacheRepository::new(data.db.pool());
    cache.get_username(uuid).await.ok().flatten()
}

pub fn text(s: impl Into<String>) -> CreateContainerComponent<'static> {
    CreateContainerComponent::TextDisplay(CreateTextDisplay::new(s.into()))
}

pub fn separator() -> CreateContainerComponent<'static> {
    CreateContainerComponent::Separator(CreateSeparator::new(true))
}
