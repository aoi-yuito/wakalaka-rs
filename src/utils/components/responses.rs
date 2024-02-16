// Copyright (c) 2024 Kawaxte
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};

use super::embeds;

pub(crate) async fn error_response_embed(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let embed = embeds::error_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) async fn warn_response_embed(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let embed = embeds::warn_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) async fn ok_response_embed(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let embed = embeds::ok_embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) async fn response_embed(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let embed = embeds::embed(&message.into());

    let response_message = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}

pub(crate) async fn response(
    message: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let response_message = CreateInteractionResponseMessage::new()
        .content(message.into())
        .ephemeral(ephemeral);
    CreateInteractionResponse::Message(response_message)
}
