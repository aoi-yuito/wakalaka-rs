// Copyright (c) 2024 Kawaxte
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::{all::colours::branding, builder::CreateEmbed};

pub(crate) fn error_embed(message: impl Into<String>) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("{}", message.into()))
        .colour(branding::RED)
}

pub(crate) fn warn_embed(message: impl Into<String>) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("{}", message.into()))
        .colour(branding::YELLOW)
}

pub(crate) fn ok_embed(message: impl Into<String>) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("{}", message.into()))
        .colour(branding::GREEN)
}

pub(crate) fn embed(message: impl Into<String>) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("{}", message.into()))
        .colour(branding::BLURPLE)
}
