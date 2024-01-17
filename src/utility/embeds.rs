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

use chrono::{NaiveDateTime, TimeZone, Utc};
use serenity::{
    all::{colours::branding, User, UserId},
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
    model::Timestamp,
};

pub(crate) fn warnings_embed(
    case_id: &i32,
    user: &User,
    user_id: &UserId,
    user_name: &String,
    moderator_id: &UserId,
    created_at: &String,
    reason: &String,
    active: &bool,
) -> CreateEmbed {
    let user_icon_url = user.avatar_url().unwrap_or(user.default_avatar_url());

    let active_status = match active {
        true => format!("✅"),
        false => format!("❌"),
    };

    let (embed_author, embed_footer) = (
        CreateEmbedAuthor::new(user_name).icon_url(user_icon_url),
        CreateEmbedFooter::new(format!("{active_status} {created_at}")),
    );

    CreateEmbed::default()
        .author(embed_author)
        .title(format!("Case #{case_id}"))
        .field("User:", format!("<@{user_id}>"), true)
        .field("Moderator:", format!("<@{moderator_id}>"), true)
        .field("Reason:", reason, false)
        .footer(embed_footer)
        .colour(branding::YELLOW)
}

pub(crate) fn warn_embed(
    user: &User,
    user_id: UserId,
    user_name: &String,
    moderator: &User,
    moderator_id: UserId,
    moderator_name: &String,
    reason: String,
    created_at: NaiveDateTime,
) -> CreateEmbed {
    let (user_icon_url, moderator_icon_url) = (
        user.avatar_url().unwrap_or(user.default_avatar_url()),
        moderator
            .avatar_url()
            .unwrap_or(moderator.default_avatar_url()),
    );

    let (embed_author, embed_footer) = (
        CreateEmbedAuthor::new(user_name).icon_url(user_icon_url),
        CreateEmbedFooter::new(moderator_name).icon_url(moderator_icon_url),
    );

    let now = Timestamp::from(Utc.from_utc_datetime(&created_at));

    CreateEmbed::default()
        .author(embed_author)
        .title("⚠️ You've been warned! ⚠️")
        .field("User:", format!("<@{user_id}>"), true)
        .field("Moderator:", format!("<@{moderator_id}>"), true)
        .field("Reason:", reason, false)
        .footer(embed_footer)
        .timestamp(now)
        .colour(branding::YELLOW)
}

pub(crate) fn suggest_embed(
    name: &String,
    avatar_url: String,
    description: &String,
    created_at: NaiveDateTime,
) -> CreateEmbed {
    let embed_author = CreateEmbedAuthor::new(name).icon_url(avatar_url);

    let now = Timestamp::from(Utc.from_utc_datetime(&created_at));

    CreateEmbed::default()
        .author(embed_author)
        .description(description)
        .color(branding::BLURPLE)
        .timestamp(Timestamp::from(now))
}

pub(crate) fn avatar_embed(name: &String, url: String) -> CreateEmbed {
    let embed_author = CreateEmbedAuthor::new(name).icon_url(url.clone());

    CreateEmbed::default().author(embed_author).image(url)
}

pub(crate) fn info_embed(icon_url: &String, constants: [&str; 6]) -> CreateEmbed {
    let author = match constants[2].split(',').next() {
        Some(value) => value,
        None => "No author found",
    };
    let embed_author = CreateEmbedAuthor::new(author).icon_url(icon_url);

    let footer = format!("Powered by Rust {}", constants[5]);
    let embed_footer = CreateEmbedFooter::new(footer);

    CreateEmbed::default()
        .author(embed_author)
        .title(format!("{} v{}", constants[0], constants[1]))
        .description(constants[3])
        .url(format!("{}/{}", constants[4], constants[0]))
        .footer(embed_footer)
        .colour(branding::BLURPLE)
}
