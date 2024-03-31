// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

use super::embeds;

pub fn error_response_embed(text: impl Into<String>, ephemeral: bool) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::error_embed(Some(format!("{text}")));

    response(text, &Some(embed), ephemeral)
}

pub fn warn_response_embed(text: impl Into<String>, ephemeral: bool) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::warn_embed(Some(format!("{text}")));

    response(text, &Some(embed), ephemeral)
}

pub fn ok_response_embed(text: impl Into<String>, ephemeral: bool) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::ok_embed(Some(format!("{text}")));

    response(text, &Some(embed), ephemeral)
}

pub fn response_embed(text: impl Into<String>, ephemeral: bool) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::embed(Some(format!("{text}")));

    response(text, &Some(embed), ephemeral)
}

pub fn response(
    text: impl Into<String>,
    embed: &Option<CreateEmbed>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let text = text.into();

    let response_message = if let Some(embed) = embed {
        CreateInteractionResponseMessage::new()
            .content(format!("{text}"))
            .embed(embed.clone())
            .ephemeral(ephemeral)
    } else {
        CreateInteractionResponseMessage::new()
            .content(format!("{text}"))
            .ephemeral(ephemeral)
    };

    CreateInteractionResponse::Message(response_message)
}
