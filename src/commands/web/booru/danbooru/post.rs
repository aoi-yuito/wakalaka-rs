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

use super::{DANBOORU_COLOUR, DANBOORU_PNG_LOGO_URL, DANBOORU_URL, POSTMAN_USER_AGENT};
use crate::{
    commands::web::booru::{self, BooruPost},
    Context,
};
use serenity::{
    all::{CommandInteraction, ResolvedOption, ResolvedValue},
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

pub(super) async fn post(
    ctx: &Context,
    options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
) -> Option<String> {
    let id = id(options)
        .await
        .unwrap_or_default()
        .parse::<i64>()
        .expect("Error while parsing ID");
    let channel_id = interaction.channel_id;

    let exists = BooruPost::post_exists(ctx, channel_id, id).await;
    if exists {
        let posts_json = format!("{DANBOORU_URL}/posts/{id}.json");

        let client = reqwest::Client::new();

        let response_text = client
            .get(&posts_json)
            .header(reqwest::header::USER_AGENT, POSTMAN_USER_AGENT) // Fucking Cloudflare...
            .send()
            .await
            .expect("Error while sending GET request")
            .text()
            .await
            .expect("Error while getting text from response");
        let response_json =
            serde_json::from_str(&response_text).expect("Error while parsing JSON from response");

        let success = booru::has_success(ctx, &response_json, channel_id).await;
        if success {
            let post = BooruPost::new(&response_json);
            let post_file_url = &post.file_url;

            let embed = BooruPost::embed(
                &post,
                DANBOORU_PNG_LOGO_URL,
                id,
                None,
                DANBOORU_URL,
                post_file_url,
                BooruPost::embed_footer(&post),
                DANBOORU_COLOUR,
            );

            let response_message = CreateInteractionResponseMessage::default();

            let message = response_message.add_embed(embed);
            let response = CreateInteractionResponse::Message(message);

            let _ = interaction.create_response(&ctx.http, response).await;
        }
    }
    None
}

async fn id(options: &[ResolvedOption<'_>]) -> Option<String> {
    for option in options {
        if let ResolvedValue::SubCommand(subcommand) = &option.value {
            let id = subcommand.get(0)?;
            if let ResolvedValue::Integer(id) = &id.value {
                return Some(id.to_string());
            }
        }
    }
    None
}
