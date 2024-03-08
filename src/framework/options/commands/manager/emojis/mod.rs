// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod add;
mod edit;
mod remove;

use crate::{
    framework::options::commands::manager::emojis::{add::add, edit::edit, remove::remove},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    subcommands("add", "edit", "remove"),
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
