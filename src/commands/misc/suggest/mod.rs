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

mod message;

use serenity::{
    builder::{ CreateCommand, CreateCommandOption },
    all::{ CommandInteraction, ResolvedOption, CommandOptionType },
};

use crate::{ commands, Context };

pub(crate) async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>]
) -> Option<String> {
    let option = commands::command_option(interaction, 0)?;
    match option.name.as_str() {
        "message" => message::message(ctx, interaction, options).await,
        _ => None,
    }
}

pub(crate) fn register() -> CreateCommand {
    CreateCommand::new("suggest")
        .description("Opens a suggestion for the community.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "message",
                "Brief summary of your suggestion."
            ).required(true)
        )
}
