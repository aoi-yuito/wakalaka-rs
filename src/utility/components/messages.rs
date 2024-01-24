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

pub async fn error_response(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let embed = embeds::error_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub async fn warn_response(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let embed = embeds::warn_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub async fn ok_response(message: impl Into<String>, ephemeral: bool) -> CreateInteractionResponse {
    let embed = embeds::ok_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub async fn info_response(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let embed = embeds::info_message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub async fn response(message: impl Into<String>, ephemeral: bool) -> CreateInteractionResponse {
    let embed = embeds::message_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub fn error_message(message: impl Into<String>) -> CreateMessage {
    let embed = embeds::error_message_embed(&message.into());

    CreateMessage::default().embed(embed)
}

pub fn warn_message(message: impl Into<String>) -> CreateMessage {
    let embed = embeds::warn_message_embed(&message.into());

    CreateMessage::default().embed(embed)
}

pub fn ok_message(message: impl Into<String>) -> CreateMessage {
    let embed = embeds::ok_message_embed(&message.into());

    CreateMessage::default().embed(embed)
}

pub fn info_message(message: impl Into<String>) -> CreateMessage {
    let embed = embeds::info_message_embed(&message.into());

    CreateMessage::default().embed(embed)
}

pub fn message(message: impl Into<String>) -> CreateMessage {
    let embed = embeds::message_embed(&message.into());

    CreateMessage::default().embed(embed)
}

pub fn error_reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let embed = embeds::error_message_embed(&message.into());

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}

pub fn warn_reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let embed = embeds::warn_message_embed(&message.into());

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}

pub fn ok_reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let embed = embeds::ok_message_embed(&message.into());

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}

pub fn info_reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let embed = embeds::info_message_embed(&message.into());

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}

pub fn reply(message: impl Into<String>, ephemeral: bool) -> CreateReply {
    let embed = embeds::message_embed(&message.into());

    CreateReply::default().embed(embed).ephemeral(ephemeral)
}
