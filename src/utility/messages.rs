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

use poise::CreateReply;
use serenity::builder::{
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
};

use super::embeds;

pub(crate) async fn error_response(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let error_embed = embeds::error_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(error_embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) async fn warn_response(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let warn_embed = embeds::warning_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(warn_embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) async fn ok_response(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let ok_embed = embeds::ok_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(ok_embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) async fn response(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let embed = embeds::message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) fn error_message(message: impl Into<String>) -> CreateMessage {
    let error_embed = embeds::error_message_embed(&message.into());

    CreateMessage::default().embed(error_embed)
}

pub(crate) fn warn_message(message: impl Into<String>) -> CreateMessage {
    let warn_embed = embeds::warning_message_embed(&message.into());

    CreateMessage::default().embed(warn_embed)
}

pub(crate) fn ok_message(message: impl Into<String>) -> CreateMessage {
    let ok_embed = embeds::ok_message_embed(&message.into());

    CreateMessage::default().embed(ok_embed)
}

pub(crate) fn message(message: impl Into<String>) -> CreateMessage {
    let embed = embeds::message_embed(&message.into());

    CreateMessage::default().embed(embed)
}

pub(crate) fn error_reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let error_embed = embeds::error_message_embed(&message.into());

    CreateReply::default()
        .embed(error_embed)
        .ephemeral(ephemeral)
}

pub(crate) fn warn_reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let warn_embed = embeds::warning_message_embed(&message.into());

    CreateReply::default()
        .embed(warn_embed)
        .ephemeral(ephemeral)
}

pub(crate) fn ok_reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let ok_embed = embeds::ok_message_embed(&message.into());

    CreateReply::default().embed(ok_embed).ephemeral(ephemeral)
}

pub(crate) fn reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let embed = embeds::message_embed(&message.into());

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}
