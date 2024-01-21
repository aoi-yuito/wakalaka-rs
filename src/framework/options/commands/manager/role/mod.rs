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
mod remove;
mod set;

use crate::{
    framework::commands::manager::role::{
        add::add, delete::delete, edit::edit, remove::remove, set::set,
    },
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("add", "delete", "edit", "remove", "set"),
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    guild_only,
    subcommand_required,
    ephemeral
)]
pub(crate) async fn role(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
