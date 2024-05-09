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

    build_response(None::<String>, &Some(embed), ephemeral)
}

pub fn build_warning_response_with_embed(
    text: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::build_embed_with_warning_notif(Some(format!("{text}")));

    build_response(None::<String>, &Some(embed), ephemeral)
}

pub fn build_success_response_with_embed(
    text: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::build_embed_with_success_notif(Some(format!("{text}")));

    build_response(None::<String>, &Some(embed), ephemeral)
}

pub fn build_response_with_embed(
    text: impl Into<String>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let text = text.into();

    let embed = embeds::build_embed(Some(format!("{text}")));

    build_response(None::<String>, &Some(embed), ephemeral)
}

pub fn build_response(
    text: Option<impl Into<String>>,
    embed: &Option<CreateEmbed>,
    ephemeral: bool,
) -> CreateInteractionResponse {
    let mut response_message = CreateInteractionResponseMessage::new().ephemeral(ephemeral);

    if let Some(text) = text {
        response_message = response_message.content(text);
    }
    if let Some(embed) = embed {
        response_message = response_message.embed(embed.clone());
    }

    CreateInteractionResponse::Message(response_message)
}
