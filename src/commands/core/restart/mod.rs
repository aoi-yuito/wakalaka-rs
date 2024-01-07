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

mod reason;

use crate::{ commands, Context };
use serenity::all::{ CommandInteraction, CommandOptionType };
use serenity::builder::{ CreateCommand, CreateCommandOption };
use serenity::model::application::ResolvedOption;

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
        "reason" => reason::reason(ctx, options),
        _ => None,
    }
}

pub(crate) fn register() -> CreateCommand {
    CreateCommand::new("restart")
        .description("Restarts yours truly.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "reason",
                "Short explanation for restarting."
            ).required(false)
        )
}
