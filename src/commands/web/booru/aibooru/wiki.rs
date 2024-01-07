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

use super::{ AIBOORU_COLOUR, AIBOORU_URL };
use crate::{ commands::web::booru::{ self, BooruWikiPages }, Context };
use serenity::{
    all::{ CommandInteraction, ResolvedOption, ResolvedValue },
    builder::{ CreateInteractionResponse, CreateInteractionResponseMessage },
};

pub(super) async fn wiki(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>]
) -> Option<String> {
    let tag = tag(options).await.unwrap_or_default().replace(" ", "_").to_lowercase();

    let channel_id = interaction.channel_id;

    let tag_exists = BooruWikiPages::tag_exists(ctx, channel_id, &tag).await;
    if tag_exists {
        let wiki_pages_show = format!("{AIBOORU_URL}/wiki_pages/{tag}");
        let wiki_pages_show_json = format!("{wiki_pages_show}.json");

        let client = reqwest::Client::new();

        let response_text = client
            .get(&wiki_pages_show_json)
            .send().await
            .expect("Expected response, but didn't find one")
            .text().await
            .expect("Expected response text, but didn't find one");
        let response_json = serde_json
            ::from_str(&response_text)
            .expect("Expected response JSON, but didn't find one");

        let success = booru::has_success(ctx, &response_json, channel_id).await;
        if success {
            let wiki_pages = BooruWikiPages::new(AIBOORU_URL, &response_json);
            let wiki_pages_title = &wiki_pages.title;

            let embed = BooruWikiPages::embed(
                &wiki_pages,
                &wiki_pages_title,
                AIBOORU_URL,
                BooruWikiPages::embed_footer(&wiki_pages),
                AIBOORU_COLOUR
            );

            let response_message = CreateInteractionResponseMessage::default().add_embed(embed);
            let response = CreateInteractionResponse::Message(response_message);

            let _ = interaction.create_response(&ctx.http, response).await;
        }
    }
    None
}

async fn tag(options: &[ResolvedOption<'_>]) -> Option<String> {
    for option in options {
        if let ResolvedValue::SubCommand(command) = &option.value {
            let tags = command.get(0)?;
            if let ResolvedValue::String(tag) = &tags.value {
                return Some(tag.to_string());
            }
        }
    }
    None
}
