// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

mod after;
mod any;
mod around;
mod before;

use crate::{
    framework::commands::manager::purge::{after::after, any::any, around::around, before::before},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("after", "any", "around", "before"),
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    guild_only,
    subcommand_required,
    ephemeral
)]
/// Delete a given amount of messages.
pub(crate) async fn purge(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
