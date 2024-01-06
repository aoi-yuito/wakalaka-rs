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

mod count;

use serenity::all::CommandInteraction;
use serenity::all::CommandOptionType;
use serenity::builder::CreateCommand;
use serenity::builder::CreateCommandOption;
use serenity::all::ResolvedOption;

use crate::commands;
use crate::Context;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>]
) -> Option<String> {
    let manage_messages = commands::has_manage_messages_permission(ctx, interaction).await;
    if !manage_messages {
        return Some("You don't have permission to delete messages!".to_string());
    }

    let command = commands::command(interaction, 0);
    match command.name.as_str() {
        "count" => count::count(ctx, interaction, options).await,
        _ => None,
    }
}

// /purge <o:count> <o:user> <o:channel>
pub fn register() -> CreateCommand {
    CreateCommand::new("purge")
        .description("Purges messages from channel.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "count",
                "Number of messages to purge."
            ).required(true)
        )
}
