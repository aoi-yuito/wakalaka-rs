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

mod add;
mod delete;
mod edit;
mod list;
mod remove;
mod set;

use crate::{
    framework::commands::manager::roles::{
        add::add, delete::delete, edit::edit, list::list, remove::remove, set::set,
    },
    check_guild_channel_restriction, Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("add", "delete", "edit", "list", "remove", "set"),
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    guild_only,
    subcommand_required,
    ephemeral
)]
pub async fn roles(ctx: Context<'_>) -> Result<(), Error> {
    let restricted = check_guild_channel_restriction!(ctx);
    if restricted {
        return Ok(());
    }

    Ok(())
}
