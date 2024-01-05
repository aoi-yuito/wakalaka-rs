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

use chrono::NaiveDate;
use serenity::{
    all::ChannelId,
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
};

#[derive(Default)]
struct Post {
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

impl Post {
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

    pub fn embed(
        post: &Post,
        icon_url: &'static str,
        post_id: i64,
        description: Option<String>,
        url: &'static str,
        image: String,
        footer: String,
        color: u32,
    ) -> CreateEmbed {
        let embed = CreateEmbed::default()
            .author(CreateEmbedAuthor::new(&post.tag_string_artist.clone()).icon_url(icon_url))
            .title(format!("Post #{post_id}"))
            .description(description.unwrap_or_default())
            .url(format!("{url}/posts/{post_id}"))
            .image(image)
            .footer(CreateEmbedFooter::new(footer))
            .color(color);
        embed
    }

    fn generate_footer(post: &Self) -> String {
        let (score, favourites, rating, file_size, file_extension, width, height, creation_date) = (
            post.score.clone(),
            post.fav_count,
            post.rating.clone(),
            files::format_file_size(post.file_size),
            post.file_ext.clone(),
            post.image_size.0,
            post.image_size.1,
            post.created_at.clone(),
        );
        format!(
            "{score} {favourites}❤️ | {rating} | {file_size} .{file_extension} ({width} x {height}) | {creation_date}")
    }

    async fn exists(ctx: &Context, channel_id: ChannelId, id: i64) -> bool {
        if id <= 0 {
            let content = format!("Post ID must be greater than 0.");
            let _ = channel_id.say(&ctx.http, &content).await;

            return false;
        }
        true
    }

    async fn has_success(
        ctx: &Context,
        response: &serde_json::Value,
        channel_id: ChannelId,
        id: i64,
    ) -> bool {
        if let Some(success) = response.get("success") {
            if !success.as_bool().unwrap_or_default() {
                let content = format!("Post `#{}` not found.", id);
                let _ = channel_id.say(&ctx.http, &content).await;

                return false;
            }
        }

        true
    }
}
