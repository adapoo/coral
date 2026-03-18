use std::io::Cursor;

use anyhow::Result;
use serenity::all::{
    CommandInteraction, Context, CreateAttachment, CreateCommand, EditInteractionResponse,
};

use crate::framework::Data;
use crate::rendering::render_prestiges;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("prestiges").description("View Bedwars star prestiges 100-5000")
}

#[allow(unused_variables)]
pub async fn run(ctx: &Context, command: &CommandInteraction, data: &Data) -> Result<()> {
    command.defer(&ctx.http).await?;

    let image = render_prestiges();

    let mut png_data = Cursor::new(Vec::new());
    image.write_to(&mut png_data, image::ImageFormat::Png)?;

    command
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new().new_attachment(CreateAttachment::bytes(
                png_data.into_inner(),
                "prestiges.png",
            )),
        )
        .await?;

    Ok(())
}
