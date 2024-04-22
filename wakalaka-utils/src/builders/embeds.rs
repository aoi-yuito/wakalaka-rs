// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{
    colours::branding::{self},
    CreateEmbed,
};

pub fn build_embed_with_error_notif(description: Option<String>) -> CreateEmbed {
    if let Some(description) = description {
        CreateEmbed::default()
            .description(format!("❌ {description}"))
            .colour(branding::RED)
    } else {
        CreateEmbed::default().colour(branding::RED)
    }
}

pub fn build_embed_with_warning_notif(description: Option<String>) -> CreateEmbed {
    if let Some(description) = description {
        CreateEmbed::default()
            .description(format!("⚠️ {description}"))
            .colour(branding::YELLOW)
    } else {
        CreateEmbed::default().colour(branding::YELLOW)
    }
}

pub fn build_embed_with_success_notif(description: Option<String>) -> CreateEmbed {
    if let Some(description) = description {
        CreateEmbed::default()
            .description(format!("✅ {description}"))
            .colour(branding::GREEN)
    } else {
        CreateEmbed::default().colour(branding::GREEN)
    }
}

pub fn build_embed(description: Option<String>) -> CreateEmbed {
    if let Some(description) = description {
        CreateEmbed::default().description(format!("{description}"))
    } else {
        CreateEmbed::default()
    }
}
