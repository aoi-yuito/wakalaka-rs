// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod create;
mod delete;
mod edit;

use crate::manager::emojis::{create::create, delete::delete, edit::edit};
use wakalaka_core::types::{Context, Throwable};

#[poise::command(
    slash_command,
    subcommands("create", "delete", "edit"),
    category = "Manager",
    required_permissions = "MANAGE_GUILD_EXPRESSIONS | CREATE_GUILD_EXPRESSIONS",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_GUILD_EXPRESSIONS | CREATE_GUILD_EXPRESSIONS",
    guild_only,
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn emojis(_ctx: Context<'_>) -> Throwable<()> {
    Ok(())
}
