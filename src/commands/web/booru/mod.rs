use crate::{util::files, Context};

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
pub mod aibooru;
pub mod danbooru;

use chrono::NaiveDate;
use regex::{Captures, Regex};
use serenity::{
    all::ChannelId,
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
};

#[derive(Default)]
struct BooruPost {
    tag_string_artist: String,
    file_url: String,
    score: String,
    fav_count: i64,
    rating: String,
    image_size: (i64, i64),
    file_size: f64,
    file_ext: String,
    created_at: String,
}

impl BooruPost {
    fn new(response: &serde_json::Value) -> Self {
        let tag_string_artist = response["tag_string_artist"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let file_url = response["file_url"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let score = match response["up_score"].as_i64().unwrap_or_default() >= 0 {
            true => format!("{0}🔼", response["up_score"].as_i64().unwrap_or_default()),
            false => format!("{0}🔽", response["down_score"].as_i64().unwrap_or_default()),
        };
        let fav_count = response["fav_count"].as_i64().unwrap_or_default();
        let rating = match response["rating"].as_str().unwrap_or_default() {
            "g" => "General😊",
            "s" => "Sensitive⚠️",
            "q" => "Questionable❓",
            "e" => "Explicit🔞",
            _ => "Unknown",
        }
        .to_string();
        let image_size = (
            response["image_width"].as_i64().unwrap_or_default(),
            response["image_height"].as_i64().unwrap_or_default(),
        );
        let file_size = response["file_size"].as_f64().unwrap_or_default();
        let file_ext = response["file_ext"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let created_at = response["created_at"]
            .as_str()
            .map(|date| {
                date.split('T')
                    .next()
                    .map(|ymd| {
                        NaiveDate::parse_from_str(ymd, "%Y-%m-%d")
                            .map(|d| d.format("%e %b %Y").to_string())
                    })
                    .unwrap_or_else(|| Ok(format!("Unknown")))
            })
            .unwrap_or_else(|| Ok(format!("Unknown")))
            .unwrap_or_default();

        Self {
            tag_string_artist,
            file_url,
            score,
            fav_count,
            rating,
            image_size,
            file_size,
            file_ext,
            created_at,
        }
    }

    fn embed(
        post: &BooruPost,
        icon_url: &'static str,
        post_id: i64,
        description: Option<String>,
        url: &'static str,
        image: &String,
        footer: String,
        color: u32,
    ) -> CreateEmbed {
        let tag_string_artist = &post.tag_string_artist;

        let embed = CreateEmbed::default()
            .author(CreateEmbedAuthor::new(tag_string_artist).icon_url(icon_url))
            .title(format!("Post #{post_id}"))
            .description(description.unwrap_or_default())
            .url(format!("{url}/posts/{post_id}"))
            .image(image)
            .footer(CreateEmbedFooter::new(footer))
            .color(color);
        embed
    }

    fn embed_footer(post: &Self) -> String {
        let (score, favourites, rating, file_size, file_extension, width, height, creation_date) = (
            &post.score,
            post.fav_count,
            &post.rating,
            files::format_file_size(post.file_size),
            &post.file_ext,
            post.image_size.0,
            post.image_size.1,
            &post.created_at,
        );
        format!(
            "{score} {favourites}❤️ | {rating} | {file_size} .{file_extension} ({width} x {height}) | {creation_date}")
    }

    async fn post_exists(ctx: &Context, channel_id: ChannelId, id: i64) -> bool {
        if id <= 0 {
            let content = format!("Post ID must be greater than 0.");
            let _ = channel_id.say(&ctx.http, &content).await;

            return false;
        }
        true
    }
}

#[derive(Default)]
struct BooruWikiPages {
    title: String,
    body: String,
    created_at: String,
    _other_names: Option<Vec<String>>,
}

impl BooruWikiPages {
    fn new(url: &'static str, response: &serde_json::Value) -> Self {
        let title = response["title"].as_str().unwrap_or_default().to_string();
        let body = response["body"].as_str().unwrap_or_default().to_string();
        let created_at = response["created_at"]
            .as_str()
            .map(|date| {
                date.split('T')
                    .next()
                    .map(|ymd| {
                        NaiveDate::parse_from_str(ymd, "%Y-%m-%d")
                            .map(|d| d.format("%e %b %Y").to_string())
                    })
                    .unwrap_or_else(|| Ok(format!("Unknown")))
            })
            .unwrap_or_else(|| Ok(format!("Unknown")))
            .unwrap_or_default();
        let other_names = response["other_names"]
            .as_array()
            .map(|names| {
                names
                    .iter()
                    .map(|name| name.as_str().unwrap_or_default().to_string())
                    .collect::<Vec<String>>()
            })
            .ok_or("Error while parsing other_names from JSON");

        Self {
            title,
            body: Self::format_body(&url, &body),
            created_at,
            _other_names: other_names.ok(),
        }
    }

    fn embed(
        wiki_pages: &BooruWikiPages,
        title: &String,
        url: &'static str,
        footer: String,
        color: u32,
    ) -> CreateEmbed {
        let body = &wiki_pages.body;

        let embed = CreateEmbed::default()
            .title(title)
            .description(Self::format_body(url, body))
            .url(url)
            .footer(CreateEmbedFooter::new(footer))
            .color(color);
        embed
    }

    fn format_body(url: &str, body: &String) -> String {
        let link_re = Regex::new(r"\[\[(.*?)\]\]").expect("Error while compiling regex");
        let symbol_re =
            Regex::new(r"\[(\w)\](.*?)\[\/(\w)\]").expect("Error while compiling regex");
        let header_re = Regex::new(r"h(\d)\.(.*)").expect("Error while compiling regex");

        let mut formatted_body = body.to_string();
        formatted_body = link_re
            .replace_all(&formatted_body, |caps: &Captures| {
                let tag = caps.get(1).map_or("", |m| m.as_str());
                let tag = tag.replace(" ", "_").to_lowercase();
                let tag = tag.split('|').next().unwrap_or_default();

                if tag.contains("tag_group:") {
                    let tag = tag.replace("tag_group:", "");

                    let wiki_pages_tag_group = format!("{url}/wiki_pages/tag_group%3A{tag}");

                    let tag = tag.split(':').last().unwrap_or_default();
                    let tag = tag.replace("_", " ");
                    format!("[{tag}]({wiki_pages_tag_group})")
                } else {
                    let wiki_pages = format!("{url}/wiki_pages/{tag}");

                    let tag = tag.replace("_", " ");
                    format!("[{tag}]({wiki_pages})")
                }
            })
            .to_string();
        formatted_body = symbol_re
            .replace_all(&formatted_body, |caps: &Captures| {
                let symbol = caps.get(1).map_or("", |m| m.as_str());
                let text = caps.get(2).map_or("", |m| m.as_str());

                let markdown_symbol = match symbol {
                    "b" => "**",
                    "i" => "*",
                    "u" => "__",
                    "s" => "~~",
                    "code" => "`",
                    "quote" => "> ",
                    "spoiler" => "||",
                    _ => "",
                };
                format!("{markdown_symbol}{text}{markdown_symbol}")
            })
            .to_string();
        formatted_body = header_re
            .replace_all(&formatted_body, |caps: &Captures| {
                let level = caps.get(1).map_or("", |m| m.as_str());
                let text = caps.get(2).map_or("", |m| m.as_str());

                let markdown_level = match level {
                    "1" => "#",
                    "2" => "#",
                    "3" => "#",
                    "4" => "#",
                    "5" => "#",
                    "6" => "#",
                    _ => "",
                };
                format!("{markdown_level} {text}")
            })
            .to_string();

        format!("{formatted_body}")
    }

    fn embed_footer(wiki_pages: &Self) -> String {
        let created_at = &wiki_pages.created_at;
        format!("{created_at}")
    }

    async fn tag_exists(ctx: &Context, channel_id: ChannelId, tag: &str) -> bool {
        if tag.is_empty() {
            let content = format!("Tag must not be empty.");
            let _ = channel_id.say(&ctx.http, &content).await;

            return false;
        }
        true
    }
}

pub async fn has_success(
    ctx: &Context,
    response: &serde_json::Value,
    channel_id: ChannelId,
) -> bool {
    if let Some(success) = response.get("success") {
        if !success.as_bool().unwrap_or_default() {
            let content = format!("Content(s) not found.");
            let _ = channel_id.say(&ctx.http, &content).await;

            return false;
        }
    }

    true
}
