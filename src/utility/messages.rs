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

pub(crate) async fn error_response(message: impl Into<String>) -> CreateInteractionResponse {
    let error_embed = embeds::error_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(error_embed)
        .ephemeral(true);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) async fn warn_response(message: impl Into<String>) -> CreateInteractionResponse {
    let warn_embed = embeds::warning_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(warn_embed)
        .ephemeral(true);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) async fn success_response(message: impl Into<String>) -> CreateInteractionResponse {
    let success_embed = embeds::ok_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(success_embed)
        .ephemeral(true);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) async fn info_response(message: impl Into<String>) -> CreateInteractionResponse {
    let info_embed = embeds::message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(info_embed)
        .ephemeral(true);
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

pub(crate) fn success_message(message: impl Into<String>) -> CreateMessage {
    let success_embed = embeds::ok_message_embed(&message.into());

    CreateMessage::default().embed(success_embed)
}

pub(crate) fn info_message(message: impl Into<String>) -> CreateMessage {
    let info_embed = embeds::message_embed(&message.into());

    CreateMessage::default().embed(info_embed)
}

pub(crate) fn error_reply(message: impl Into<String>) -> CreateReply {
    let error_embed = embeds::error_message_embed(&message.into());

    CreateReply::default().embed(error_embed).ephemeral(true)
}

pub(crate) fn warn_reply(message: impl Into<String>) -> CreateReply {
    let warn_embed = embeds::warning_message_embed(&message.into());

    CreateReply::default().embed(warn_embed).ephemeral(true)
}

pub(crate) fn success_reply(message: impl Into<String>) -> CreateReply {
    let success_embed = embeds::ok_message_embed(&message.into());

    CreateReply::default().embed(success_embed).ephemeral(true)
}

pub(crate) fn info_reply(message: impl Into<String>) -> CreateReply {
    let info_embed = embeds::message_embed(&message.into());

    CreateReply::default().embed(info_embed).ephemeral(true)
}
