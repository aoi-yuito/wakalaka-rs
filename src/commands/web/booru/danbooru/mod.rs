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

mod post;
mod wiki;

use crate::{commands, Context};
use serenity::{
    all::{CommandInteraction, CommandOptionType, ResolvedOption},
    builder::{CreateCommand, CreateCommandOption},
};

const DANBOORU_URL: &str = "https://danbooru.donmai.us";
const DANBOORU_PNG_LOGO_URL: &str =
    "https://danbooru.donmai.us/packs/static/images/danbooru-logo-128x128-ea111b6658173e847734.png";

const DANBOORU_COLOUR: u32 = 0xAC8A64;

const POSTMAN_USER_AGENT: &str = "PostmanRuntime/7.36.0";

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>],
) -> Option<String> {
    let command = commands::command(interaction, 0);
    match command.name.as_str() {
        "post" => post::post(ctx, options, interaction).await,
        "wiki" => wiki::wiki(ctx, options, interaction).await,
        _ => None,
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("danbooru")
        .description(format!("Provides interactibility with {DANBOORU_URL:?}").as_str())
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "post",
                "Fetches info for post.",
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::Integer, "id", "Index of post.")
                    .required(true),
            ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "wiki",
                "Previews a wiki page from Danbooru.",
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::String, "tag", "Tag of wiki page.")
                    .required(true),
            ),
        )
}
