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

use serenity::{
    all::{ CommandInteraction, colours::branding },
    builder::{
        CreateEmbed,
        CreateEmbedAuthor,
        CreateEmbedFooter,
        CreateInteractionResponseMessage,
        CreateInteractionResponse,
    },
};

use crate::Context;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const RUST_VERSION: &str = env!("CARGO_PKG_RUST_VERSION");

const GITHUB_URL: &str = "https://github.com/Kawaxte/wakalaka-rs";

#[derive(Default)]
struct BotInfo {
    name: String,
    authors: Vec<String>,
    description: String,
    rust_version: String,
}

impl BotInfo {
    fn new() -> Self {
        Self {
            name: format!("{NAME} v{VERSION}"),
            authors: AUTHORS.split(",")
                .map(|s| s.to_string())
                .collect(),
            description: format!("{DESCRIPTION}"),
            rust_version: format!("{RUST_VERSION}"),
        }
    }

    fn embed(&self, icon_url: &String) -> CreateEmbed {
        let name = &self.name;
        let author = &self.authors.join(", ");
        let description = &self.description;

        let embed = CreateEmbed::default()
            .author(CreateEmbedAuthor::new(author).icon_url(icon_url))
            .title(name)
            .description(description)
            .url(GITHUB_URL)
            .footer(CreateEmbedFooter::new(self.embed_footer()))
            .color(branding::RED);
        embed
    }

    fn embed_footer(&self) -> String {
        let rust_version = &self.rust_version;
        format!("Powered by Rust {rust_version}")
    }
}

pub(super) async fn info(ctx: &Context, interaction: &CommandInteraction) -> Option<String> {
    let bot_user = ctx.http
        .get_current_user().await
        .expect("Expected current user, but didn't find one");
    let bot_avatar_url = bot_user.avatar_url().unwrap_or_default();

    let embed = BotInfo::new().embed(&bot_avatar_url);

    let response_message = CreateInteractionResponseMessage::default().add_embed(embed);
    let response = CreateInteractionResponse::Message(response_message);

    match interaction.create_response(&ctx.http, response).await {
        Ok(_) => None,
        Err(why) => Some(format!("Error while creating response: {why}")),
    }
}
