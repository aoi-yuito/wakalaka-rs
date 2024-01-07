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

mod command;

use serenity::{ builder::{ CreateCommand, CreateCommandOption }, all::CommandOptionType };
use serenity::all::CommandInteraction;
use serenity::all::ResolvedOption;

use crate::{ Context, commands };

pub(crate) async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>]
) -> Option<String> {
    let administrator = commands::has_administrator_permission(ctx, interaction).await;
    if !administrator {
        return Some(format!("You don't have permission(s) to execute this command!"));
    }

    let command = commands::command(interaction, 0);
    match command.name.as_str() {
        "command" => command::command(ctx, interaction, options).await,
        _ => None,
    }
}

pub(crate) fn register() -> CreateCommand {
    CreateCommand::new("reload")
        .description("Reloads a command.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "command",
                "Name of command to reload."
            ).required(true)
        )
}
