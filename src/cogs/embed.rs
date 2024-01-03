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
use crate::uses::*;

pub fn is_embed_image_or_thumbnail(msg: &Message) -> bool {
    for embed in &msg.embeds {
        if embed.image.is_some() || embed.thumbnail.is_some() {
            return true;
        }
    }
    false
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

pub fn create_embed_for_metadata(title: String, color: u32) -> CreateEmbed {
    let embed = CreateEmbed::default().title(title).color(color);
    embed
}
