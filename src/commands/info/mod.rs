// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod about;
mod user;

use crate::commands::info::user::user;
use wakalaka_core::types::{Command, Context, Throwable};

#[poise::command(
    slash_command,
    subcommands("user"),
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn info(_ctx: Context<'_>) -> Throwable<()> {
    Ok(())
}

pub async fn commands() -> Vec<Command> {
    vec![about::about(), info()]
}
