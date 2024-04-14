// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use serenity::all::CreateEmbed;

use super::embeds;

pub fn build_error_reply_with_embed(text: impl Into<String>, ephemeral: bool) -> CreateReply {
    let text = text.into();

    let embed = embeds::build_embed_with_error_notif(Some(format!("{text}")));

    build_reply_with_optional_embed(text, &Some(embed), ephemeral)
}

pub fn build_warning_reply_with_embed(text: impl Into<String>, ephemeral: bool) -> CreateReply {
    let text = text.into();

    let embed = embeds::build_embed_with_warning_notif(Some(format!("{text}")));

    build_reply_with_optional_embed(text, &Some(embed), ephemeral)
}

pub fn build_success_reply_with_embed(text: impl Into<String>, ephemeral: bool) -> CreateReply {
    let text = text.into();

    let embed = embeds::build_embed_with_success_notif(Some(format!("{text}")));

    build_reply_with_optional_embed(text, &Some(embed), ephemeral)
}

pub fn build_reply_with_embed(text: impl Into<String>, ephemeral: bool) -> CreateReply {
    let text = text.into();

    let embed = embeds::build_embed(Some(format!("{text}")));

    build_reply_with_optional_embed(text, &Some(embed), ephemeral)
}

pub fn build_reply_with_optional_embed(
    text: impl Into<String>,
    embed: &Option<CreateEmbed>,
    ephemeral: bool,
) -> CreateReply {
    let text = text.into();

    if let Some(embed) = embed {
        CreateReply::default()
            .content(text)
            .embed(embed.clone())
            .ephemeral(ephemeral)
    } else {
        CreateReply::default().content(text).ephemeral(ephemeral)
    }
}
