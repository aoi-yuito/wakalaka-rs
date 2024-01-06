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

mod user;

use crate::{ commands, Context };
use serenity::{
    all::{ CommandInteraction, CommandOptionType },
    builder::{ CreateCommand, CreateCommandOption },
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Option<String> {
    let command = commands::command(interaction, 0);
    match command.name.as_str() {
        "user" => user::user(ctx, interaction).await,
        _ => None,
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("avatar")
        .description("Fetches user's avatar.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "ID (or mention) of user to fetch avatar from."
            ).required(true)
        )
}
