// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateEmbed, CreateMessage};

use super::embeds;

pub fn error_message_embed(text: impl Into<String>) -> CreateMessage {
    let text = text.into();

    let embed = embeds::error_embed(Some(format!("{text}")));

    message(text, &Some(embed))
}

pub fn warn_message_embed(text: impl Into<String>) -> CreateMessage {
    let text = text.into();

    let embed = embeds::warn_embed(Some(format!("{text}")));

    message(text, &Some(embed))
}

pub fn ok_message_embed(text: impl Into<String>) -> CreateMessage {
    let text = text.into();

    let embed = embeds::ok_embed(Some(format!("{text}")));

    message(text, &Some(embed))
}

pub fn message_embed(text: impl Into<String>) -> CreateMessage {
    let text = text.into();

    let embed = embeds::embed(Some(format!("{text}")));

    message(text, &Some(embed))
}

pub fn message(text: impl Into<String>, embed: &Option<CreateEmbed>) -> CreateMessage {
    let text = text.into();

    if let Some(embed) = embed {
        CreateMessage::default().content(text).embed(embed.clone())
    } else {
        CreateMessage::default().content(text)
    }
}
