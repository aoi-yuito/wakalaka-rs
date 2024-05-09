// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateEmbed, CreateMessage};

use super::embeds;

pub fn build_error_message_with_embed(text: impl Into<String>) -> CreateMessage {
    let text = text.into();

    let embed = embeds::build_embed_with_error_notif(Some(format!("{text}")));

    build_message(None::<String>, &Some(embed))
}

pub fn build_warning_message_with_embed(text: impl Into<String>) -> CreateMessage {
    let text = text.into();

    let embed = embeds::build_embed_with_warning_notif(Some(format!("{text}")));

    build_message(None::<String>, &Some(embed))
}

pub fn build_success_message_with_embed(text: impl Into<String>) -> CreateMessage {
    let text = text.into();

    let embed = embeds::build_embed_with_success_notif(Some(format!("{text}")));

    build_message(None::<String>, &Some(embed))
}

pub fn build_message_with_embed(text: impl Into<String>) -> CreateMessage {
    let text = text.into();

    let embed = embeds::build_embed(Some(format!("{text}")));

    build_message(None::<String>, &Some(embed))
}

pub fn build_message(
    text: Option<impl Into<String>>,
    embed: &Option<CreateEmbed>,
) -> CreateMessage {
    let mut message = CreateMessage::default();

    if let Some(text) = text {
        message = message.content(text);
    }
    if let Some(embed) = embed {
        message = message.embed(embed.clone());
    }

    message
}
