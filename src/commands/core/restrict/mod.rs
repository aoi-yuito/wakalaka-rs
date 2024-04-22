// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod guild;
mod user;

use crate::commands::core::restrict::{guild::guild, user::user};
use wakalaka_core::types::{Context, Throwable};

#[poise::command(
    slash_command,
    subcommands("guild", "user"),
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    required_bot_permissions = "SEND_MESSAGES",
    owners_only,
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn restrict(_ctx: Context<'_>) -> Throwable<()> {
    Ok(())
}
