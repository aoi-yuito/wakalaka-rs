// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod add;
mod create;
mod delete;
mod edit;
mod remove;

use crate::commands::manager::roles::{
    add::add, create::create, delete::delete, edit::edit, remove::remove,
};
use wakalaka_core::types::{Context, Throwable};

#[poise::command(
    slash_command,
    subcommands("add", "create", "delete", "edit", "remove"),
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    required_bot_permissions = "MANAGE_ROLES",
    guild_only,
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn roles(_ctx: Context<'_>) -> Throwable<()> {
    Ok(())
}
