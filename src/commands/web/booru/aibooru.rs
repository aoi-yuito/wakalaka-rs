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

use super::{BooruPost, BooruWikiPages};
use crate::{commands, Context};
use serenity::{
    all::{CommandInteraction, CommandOptionType, ResolvedOption, ResolvedValue},
    builder::{
        CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
};

const AIBOORU_URL: &str = "https://aibooru.online";
const AIBOORU_PNG_LOGO_URL: &str =
    "https://aibooru.online/packs/static/images/danbooru-logo-128x128-5dfe4b292bd64a786b41.png";

const AIBOORU_COLOUR: u32 = 0x77B91E;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>],
) -> Option<String> {
    let command = commands::command(interaction, 0);
    match command.name.as_str() {
        "post" => post(ctx, options, interaction).await,
        "wiki" => wiki(ctx, options, interaction).await,
        _ => None,
    }
}

async fn wiki(
    ctx: &Context,
    options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
) -> Option<String> {
    let tag = tag(options)
        .await
        .unwrap_or_default()
        .replace(" ", "_")
        .to_lowercase();
    let channel_id = interaction.channel_id;
    let tag_exists = BooruWikiPages::tag_exists(ctx, channel_id, &tag).await;
    if tag_exists {
        let wiki_pages_show = format!("{AIBOORU_URL}/wiki_pages/{tag}");
        let wiki_pages_show_json = format!("{wiki_pages_show}.json");

        let client = reqwest::Client::new();

        let response_text = client
            .get(&wiki_pages_show_json)
            .send()
            .await
            .expect("Error while sending GET request")
            .text()
            .await
            .expect("Error while getting text from response");
        let response_json =
            serde_json::from_str(&response_text).expect("Error while parsing JSON from response");

        let success = super::has_success(ctx, &response_json, channel_id).await;
        if success {
            let wiki_pages = BooruWikiPages::new(AIBOORU_URL, &response_json);
            let wiki_pages_title = &wiki_pages.title;

            let embed = BooruWikiPages::embed(
                &wiki_pages,
                &wiki_pages_title,
                AIBOORU_URL,
                BooruWikiPages::embed_footer(&wiki_pages),
                AIBOORU_COLOUR,
            );

            let response_message = CreateInteractionResponseMessage::default();

            let message = response_message.add_embed(embed);
            let response = CreateInteractionResponse::Message(message);

            let _ = interaction.create_response(&ctx.http, response).await;
        }
    }
    None
}

async fn post(
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

    let post_exists = BooruPost::post_exists(ctx, channel_id, id).await;
    if post_exists {
        let posts_show_json = format!("{AIBOORU_URL}/posts/{id}.json");

        let client = reqwest::Client::new();

        let response_text = client
            .get(&posts_show_json)
            .send()
            .await
            .expect("Error while sending GET request")
            .text()
            .await
            .expect("Error while getting text from response");
        let response_json =
            serde_json::from_str(&response_text).expect("Error while parsing JSON from response");

        let success = super::has_success(ctx, &response_json, channel_id).await;
        if success {
            let post = BooruPost::new(&response_json);
            let post_file_url = &post.file_url;

            let embed = BooruPost::embed(
                &post,
                AIBOORU_PNG_LOGO_URL,
                id,
                None,
                AIBOORU_URL,
                post_file_url,
                BooruPost::embed_footer(&post),
                AIBOORU_COLOUR,
            );

            let response_message = CreateInteractionResponseMessage::default();

            let message = response_message.add_embed(embed);
            let response = CreateInteractionResponse::Message(message);

            let _ = interaction.create_response(&ctx.http, response).await;
        }
    }
    None
}

async fn tag(options: &[ResolvedOption<'_>]) -> Option<String> {
    for option in options {
        if let ResolvedValue::SubCommand(subcommand) = &option.value {
            let tags = subcommand.get(0)?;
            if let ResolvedValue::String(tags) = &tags.value {
                return Some(tags.to_string());
            }
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

pub fn register() -> CreateCommand {
    CreateCommand::new("aibooru")
        .description(format!("Provides interactibility with {AIBOORU_URL:?}").as_str())
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "post",
                "Previews a post from AIBooru.",
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
                "Previews a tag from AIBooru wiki pages.",
            )
            .add_sub_option(
                CreateCommandOption::new(CommandOptionType::String, "tag", "Name of tag.")
                    .required(true),
            ),
        )
}
