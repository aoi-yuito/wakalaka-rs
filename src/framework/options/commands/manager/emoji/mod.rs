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

use crate::{
    framework::commands::manager::emoji::{add::add, delete::delete},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("add", "delete"),
    category = "Manager",
    required_permissions = "MANAGE_GUILD_EXPRESSIONS | CREATE_GUILD_EXPRESSIONS",
    guild_only,
    subcommand_required,
    ephemeral
)]
pub async fn emoji(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
