/**
 * Copyright (C) 2024 Kasutaja
 *
 * wakalaka-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * wakalaka-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::util::uses::*;

pub mod aibooru;

pub const AIBOORU_URL: &str = "https://aibooru.online";
pub const AIBOORU_LOGO_PNG: &str =
    "https://aibooru.online/packs/static/images/danbooru-logo-128x128-5dfe4b292bd64a786b41.png";

#[derive(Default)]
pub struct Post {
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
    async fn is_success(
        ctx: crate::Context<'_>,
        response: &serde_json::Value,
        mut _reply: CreateReply,
        id: i64,
    ) -> Result<bool, crate::Error> {
        if let Some(success) = response.get("success") {
            if !success.as_bool().unwrap_or_default() {
                _reply = CreateReply {
                    content: Some(format!("Post `#{}` not found.", id)),
                    ..Default::default()
                };

                ctx.send(_reply).await?;
                return Ok(false);
            }
        }
        Ok(true)
    }

    async fn exists(
        ctx: crate::Context<'_>,
        mut _reply: CreateReply,
        id: i64,
    ) -> Result<bool, crate::Error> {
        if id <= 0 {
            _reply = CreateReply {
                content: Some("Post ID must be greater than 0.".to_string()),
                ..Default::default()
            };

            ctx.send(_reply).await?;
            return Ok(false);
        }
        Ok(true)
    }

    fn extract_post_data(response: &serde_json::Value) -> Self {
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
                    .unwrap_or_else(|| Ok("Unknown".to_string()))
            })
            .unwrap_or_else(|| Ok("Unknown".to_string()))
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

    fn generate_footer(post: &Self) -> String {
        format!(
            "{0} {1}❤️ | {2} | {3} .{4} ({5} x {6}) | {7}",
            post.score,
            post.fav_count,
            post.rating,
            files::format_size(post.file_size),
            post.file_ext,
            post.image_size.0,
            post.image_size.1,
            post.created_at
        )
    }

    fn create_embed(
        post: &Self,
        id: i64,
        description: Option<&'static str>,
        url: &'static str,
    ) -> CreateEmbed {
        CreateEmbed::default()
            .author(
                CreateEmbedAuthor::new(post.tag_string_artist.clone()).icon_url(AIBOORU_LOGO_PNG),
            )
            .title(format!("Post #{id}"))
            .description(description.unwrap_or_default())
            .url(format!("{url}/posts/{id}"))
            .image(post.file_url.clone())
            .footer(CreateEmbedFooter::new(Self::generate_footer(post)))
            .color(0x7EB900)
    }
}
