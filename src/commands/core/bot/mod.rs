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

use crate::commands;
use crate::Context;

use serenity::all::CommandInteraction;
use serenity::all::CommandOptionType;
use serenity::builder::CreateCommand;
use serenity::builder::CreateCommandOption;

mod info;

pub(crate) async fn run(ctx: &Context, interaction: &CommandInteraction) -> Option<String> {
    let option = commands::command_option(interaction, 0)?;
    match option.name.as_str() {
        "info" => info::info(ctx, interaction).await,
        _ => None,
    }
}

pub(crate) fn register() -> CreateCommand {
    CreateCommand::new("bot")
        .description("Commands related to yours truly.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "info", "Bot information.")
        )
}
