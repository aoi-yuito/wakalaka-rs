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

pub fn is_message_embed(msg: &Message) -> bool {
    !msg.embeds.is_empty()
}

pub fn is_embed_containing_image(msg: &Message) -> bool {
    for embed in &msg.embeds {
        if embed.image.is_some() || embed.thumbnail.is_some() {
            return true;
        }
    }
    false
}

#[derive(Default)]
pub struct Embed {
    author: Option<String>,
    title: Option<String>,
    description: Option<String>,
    url: Option<&'static str>,
    image: Option<String>,
    thumbnail: Option<String>,
    footer: Option<String>,
    color: Option<u32>,
}

impl Embed {
    pub fn create_embed(
        title: Option<&'static str>,
        description: Option<&'static str>,
        url: Option<&'static str>,
        image: Option<&'static str>,
        thumbnail: Option<&'static str>,
        footer: Option<&'static str>,
        color: Option<u32>,
    ) -> CreateEmbed {
        let embed = CreateEmbed::default()
            .title(title.unwrap_or_default())
            .description(description.unwrap_or_default())
            .url(url.unwrap_or_default())
            .image(image.unwrap_or_default())
            .thumbnail(thumbnail.unwrap_or_default())
            .footer(CreateEmbedFooter::new(footer.unwrap_or_default()))
            .color(color.unwrap_or_default());
        embed
    }

    pub fn create_embed_for_booru(
        post: &Post,
        icon_url: &'static str,
        id: i64,
        description: Option<String>,
        url: &'static str,
        image: String,
        footer: String,
        color: u32,
    ) -> CreateEmbed {
        let embed = CreateEmbed::default()
            .author(CreateEmbedAuthor::new(&post.tag_string_artist.clone()).icon_url(icon_url))
            .title(format!("Post #{}", id))
            .description(description.unwrap_or_default())
            .url(format!("{url}/posts/{id}"))
            .image(image)
            .footer(CreateEmbedFooter::new(footer))
            .color(color);
        embed
    }

    pub fn embed_urls(msg: &Message) -> Vec<String> {
        let mut urls = Vec::new();

        for embed in &msg.embeds {
            if let Some(image) = &embed.image {
                urls.push(image.url.clone());
            }
            if let Some(thumbnail) = &embed.thumbnail {
                urls.push(thumbnail.url.clone());
            }
        }
        urls
    }
}
