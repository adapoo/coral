//! Subscription commands for list subscribers.

use anyhow::Result;
use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::framework::Data;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("subscribe").description("Subscribe to a user list")
    // TODO: add invite key option
}

pub async fn run(_ctx: &Context, _command: &CommandInteraction, _data: &Data) -> Result<()> {
    // TODO: validate invite key, add subscription
    todo!()
}

pub fn register_unsubscribe() -> CreateCommand<'static> {
    CreateCommand::new("unsubscribe").description("Unsubscribe from a user list")
    // TODO: add list selection
}

pub async fn run_unsubscribe(
    _ctx: &Context,
    _command: &CommandInteraction,
    _data: &Data,
) -> Result<()> {
    // TODO: remove subscription
    todo!()
}
