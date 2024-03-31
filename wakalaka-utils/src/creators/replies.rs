// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use serenity::all::CreateEmbed;

use super::embeds;

pub fn error_reply_embed(text: impl Into<String>, ephemeral: bool) -> CreateReply {
    let text = text.into();

    let embed = embeds::error_embed(Some(format!("{text}")));

    reply(text, &Some(embed), ephemeral)
}

pub fn warn_reply_embed(text: impl Into<String>, ephemeral: bool) -> CreateReply {
    let text = text.into();

    let embed = embeds::warn_embed(Some(format!("{text}")));

    reply(text, &Some(embed), ephemeral)
}

pub fn ok_reply_embed(text: impl Into<String>, ephemeral: bool) -> CreateReply {
    let text = text.into();

    let embed = embeds::ok_embed(Some(format!("{text}")));

    reply(text, &Some(embed), ephemeral)
}

pub fn reply_embed(text: impl Into<String>, ephemeral: bool) -> CreateReply {
    let text = text.into();

    let embed = embeds::embed(Some(format!("{text}")));

    reply(text, &Some(embed), ephemeral)
}

pub fn reply(text: impl Into<String>, embed: &Option<CreateEmbed>, ephemeral: bool) -> CreateReply {
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
