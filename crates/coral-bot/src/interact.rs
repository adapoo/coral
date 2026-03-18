use anyhow::Result;
use serenity::all::{
    CommandInteraction, Component, ComponentInteraction, Context, CreateComponent, CreateContainer,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateSectionComponent,
    CreateTextDisplay, LabelComponent, MessageFlags, ModalInteraction,
};

use crate::utils::text;

const COLOR_ERROR: u32 = 0xED4245;

pub fn section_text(s: &str) -> CreateSectionComponent<'static> {
    CreateSectionComponent::TextDisplay(CreateTextDisplay::new(s.to_string()))
}

pub fn parse_id(custom_id: &str) -> Option<u64> {
    custom_id
        .splitn(2, ':')
        .nth(1)?
        .split(':')
        .next()?
        .parse()
        .ok()
}

pub fn parse_ids(custom_id: &str) -> Option<(u64, String)> {
    let payload = custom_id.splitn(2, ':').nth(1)?;
    let mut parts = payload.splitn(2, ':');
    let first: u64 = parts.next()?.parse().ok()?;
    let second = parts.next()?.to_string();
    Some((first, second))
}

pub fn parse_compound_id(custom_id: &str) -> Option<(u64, u64)> {
    let payload = custom_id.splitn(2, ':').nth(1)?;
    let mut parts = payload.split(':');
    let first: u64 = parts.next()?.parse().ok()?;
    let second: u64 = parts.next()?.parse().ok()?;
    Some((first, second))
}

pub fn extract_modal_value(components: &[Component], field_id: &str) -> String {
    for component in components {
        if let Component::Label(label) = component {
            if let LabelComponent::InputText(input) = &label.component {
                if input.custom_id == field_id {
                    return input.value.as_deref().unwrap_or("").to_string();
                }
            }
        }
    }
    String::new()
}

pub async fn update_message(
    ctx: &Context,
    component: &ComponentInteraction,
    components: Vec<CreateComponent<'static>>,
) -> Result<()> {
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
    Ok(())
}

pub async fn update_modal(
    ctx: &Context,
    modal: &ModalInteraction,
    components: Vec<CreateComponent<'static>>,
) -> Result<()> {
    modal
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(components),
            ),
        )
        .await?;
    Ok(())
}

pub async fn send_error(ctx: &Context, command: &CommandInteraction, message: &str) -> Result<()> {
    let container = CreateComponent::Container(
        CreateContainer::new(vec![text(format!("## Error\n{message}"))]).accent_color(COLOR_ERROR),
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

pub async fn send_component_error(
    ctx: &Context,
    component: &ComponentInteraction,
    message: &str,
) -> Result<()> {
    let container = CreateComponent::Container(
        CreateContainer::new(vec![text(format!("## Error\n{message}"))]).accent_color(COLOR_ERROR),
    );

    component
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

pub async fn send_modal_error(
    ctx: &Context,
    modal: &ModalInteraction,
    message: &str,
) -> Result<()> {
    modal
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
