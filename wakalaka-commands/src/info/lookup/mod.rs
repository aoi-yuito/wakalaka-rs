// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod guild;
mod user;

use crate::info::lookup::{guild::guild, user::user};
use wakalaka_core::types::{Context, Throwable};

#[poise::command(
    slash_command,
    subcommands("guild", "user"),
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn lookup(_ctx: Context<'_>) -> Throwable<()> {
    Ok(())
}
