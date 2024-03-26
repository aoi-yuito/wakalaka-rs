// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::builder::CreateMessage;

use super::embeds;

pub(crate) fn error_message_embed(message: impl Into<String>) -> CreateMessage {
    let message = message.into();

    let embed = embeds::error_embed(&message);

    CreateMessage::default().embed(embed)
}

pub(crate) fn warn_message_embed(message: impl Into<String>) -> CreateMessage {
    let message = message.into();

    let embed = embeds::warn_embed(&message);

    CreateMessage::default().embed(embed)
}

pub(crate) fn ok_message_embed(message: impl Into<String>) -> CreateMessage {
    let message = message.into();

    let embed = embeds::ok_embed(&message);

    CreateMessage::default().embed(embed)
}

pub(crate) fn message_embed(message: impl Into<String>) -> CreateMessage {
    let message = message.into();

    let embed = embeds::embed(&message);

    CreateMessage::default().embed(embed)
}

pub(crate) fn message(message: impl Into<String>) -> CreateMessage {
    let message = message.into();

    CreateMessage::default().content(message)
}
