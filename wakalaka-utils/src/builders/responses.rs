// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

use super::embeds;

pub fn build_error_response_with_embed(
    text: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::build_embed_with_error_notif(Some(format!("{text}")));

    build_response_with_optional_embed(text, &Some(embed), ephemeral)
}

pub fn build_warning_response_with_embed(
    text: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::build_embed_with_warning_notif(Some(format!("{text}")));

    build_response_with_optional_embed(text, &Some(embed), ephemeral)
}

pub fn build_success_response_with_embed(
    text: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::build_embed_with_success_notif(Some(format!("{text}")));

    build_response_with_optional_embed(text, &Some(embed), ephemeral)
}

pub fn build_response_with_embed(
    text: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::build_embed(Some(format!("{text}")));

    build_response_with_optional_embed(text, &Some(embed), ephemeral)
}

pub fn build_response_with_optional_embed(
    text: impl Into<String>,
    embed: &Option<CreateEmbed>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let text = text.into();

    let response_message = if let Some(embed) = embed {
        CreateInteractionResponseMessage::new()
            .embed(embed.clone())
            .ephemeral(ephemeral)
    } else {
        CreateInteractionResponseMessage::new()
            .content(format!("{text}"))
            .ephemeral(ephemeral)
    };

    CreateInteractionResponse::Message(response_message)
}
