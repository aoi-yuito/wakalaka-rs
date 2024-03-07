// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod add;
mod assign;
mod edit;
mod remove;
mod unassign;

use crate::{
    framework::options::commands::manager::roles::{
        add::add, assign::assign, edit::edit, remove::remove, unassign::unassign,
    },
    Context, Error,
};

#[poise::command(
    slash_command,
    subcommands("add", "assign", "edit", "remove", "unassign"),
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    required_bot_permissions = "MANAGE_ROLES",
    guild_only,
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn roles(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
