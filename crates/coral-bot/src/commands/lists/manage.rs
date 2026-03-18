//! List management commands for list owners.

use anyhow::Result;
use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::framework::Data;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("list").description("Manage your user lists")
    // TODO: subcommands: create, delete, add, remove, invite, revoke
}

pub async fn run(_ctx: &Context, _command: &CommandInteraction, _data: &Data) -> Result<()> {
    // TODO: handle subcommands
    todo!()
}
