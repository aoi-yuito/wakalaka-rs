// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod after;
mod around;
mod before;

use crate::{
    framework::options::commands::manager::purge::{after::after, around::around, before::before},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    subcommands("after", "around", "before"),
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    required_bot_permissions = "MANAGE_MESSAGES",
    guild_only,
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn purge(_ctx: Context<'_>) -> Throwable<()> {
    Ok(())
}
