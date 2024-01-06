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

const DANBOORU_URL: &str = "https://danbooru.donmai.us";
const DANBOORU_LOGO_PNG: &str =
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
        let wiki_pages_show = format!("{DANBOORU_URL}/wiki_pages/{tag}");
        let wiki_pages_show_json = format!("{wiki_pages_show}.json");

        let client = reqwest::Client::new();

        let response_text = client
            .get(&wiki_pages_show_json)
            .header(reqwest::header::USER_AGENT, POSTMAN_USER_AGENT) // Fucking Cloudflare...
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
            let wiki_pages = BooruWikiPages::new(DANBOORU_URL, &response_json);
            let wiki_pages_title = &wiki_pages.title;

            let embed = BooruWikiPages::embed(
                &wiki_pages,
                &wiki_pages_title,
                DANBOORU_URL,
                BooruWikiPages::embed_footer(&wiki_pages),
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

        let success = super::has_success(ctx, &response_json, channel_id).await;
        if success {
            let post = BooruPost::new(&response_json);
            let post_file_url = &post.file_url;

            let embed = BooruPost::embed(
                &post,
                DANBOORU_LOGO_PNG,
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
    CreateCommand::new("danbooru")
        .description(format!("Provides interactibility with {DANBOORU_URL:?}").as_str())
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "post",
                "Previews a post from Danbooru.",
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
