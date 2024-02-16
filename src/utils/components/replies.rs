// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;

use super::embeds;

pub(crate) fn error_reply_embed(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let message = message.into();

    let embed = embeds::error_embed(&message);

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}

pub(crate) fn warn_reply_embed(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let message = message.into();

    let embed = embeds::warn_embed(&message);

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}

pub(crate) fn ok_reply_embed(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let message = message.into();

    let embed = embeds::ok_embed(&message);

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}

pub(crate) fn reply_embed(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let message = message.into();

    let embed = embeds::embed(&message);

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}

pub(crate) fn reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let message = message.into();

    CreateReply::default().content(message).ephemeral(ephemeral)
}
