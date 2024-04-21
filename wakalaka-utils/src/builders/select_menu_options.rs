// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateSelectMenuOption, GuildId, ReactionType, UserId};
use sqlx::types::chrono::NaiveDateTime;

pub async fn build_restricted_guild_select_menu_option(
    guild_id: &GuildId,
    reason: &String,
    created_at: &NaiveDateTime,
) -> CreateSelectMenuOption {
    let simple_created_at = format!("{}", created_at.format("%b %d, %Y"));

    CreateSelectMenuOption::new(
        format!("{guild_id} ({simple_created_at})"),
        format!("{guild_id}"),
    )
    .description(reason.trim())
    .emoji(ReactionType::Unicode(format!("⛔")))
}

pub fn build_select_menu_option_with_emoji(
    label: impl Into<String>,
    value: impl Into<String>,
    description: impl Into<String>,
    emoji: &str,
) -> CreateSelectMenuOption {
    CreateSelectMenuOption::new(label, value)
        .description(description)
        .emoji(ReactionType::Unicode(format!("{emoji}")))
}
