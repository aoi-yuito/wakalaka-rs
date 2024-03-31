// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{
    colours::branding::{self},
    CreateEmbed,
};

pub fn error_embed(description: Option<String>) -> CreateEmbed {
    if let Some(description) = description {
        CreateEmbed::default()
            .description(format!("❌ {description}"))
            .colour(branding::RED)
    } else {
        CreateEmbed::default().colour(branding::RED)
    }
}

pub fn warn_embed(description: Option<String>) -> CreateEmbed {
    if let Some(description) = description {
        CreateEmbed::default()
            .description(format!("⚠️ {description}"))
            .colour(branding::YELLOW)
    } else {
        CreateEmbed::default().colour(branding::YELLOW)
    }
}

pub fn ok_embed(description: Option<String>) -> CreateEmbed {
    if let Some(description) = description {
        CreateEmbed::default()
            .description(format!("✅ {description}"))
            .colour(branding::GREEN)
    } else {
        CreateEmbed::default().colour(branding::GREEN)
    }
}

pub fn embed(description: Option<String>) -> CreateEmbed {
    if let Some(description) = description {
        CreateEmbed::default().description(format!("{description}"))
    } else {
        CreateEmbed::default()
    }
}
