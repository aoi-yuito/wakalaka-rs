// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateSelectMenuOption, GuildId, ReactionType, UserId};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

use wakalaka_core::types::Context;

pub async fn build_restricted_guild_select_menu_option(
    guild_id: &GuildId,
    reason: &String,
    created_at: &NaiveDateTime,
) -> CreateSelectMenuOption {
    let simple_created_at = format!("{}", created_at.format("%b %d, %Y"));

    build_select_menu_option_with_emoji(
        format!("{guild_id} - {simple_created_at}"),
        format!("{guild_id}"),
        reason.trim(),
        "⛔",
    )
}

pub async fn build_warning_select_menu_option(
    ctx: Context<'_>,
    uuid: &Uuid,
    moderator_id: &UserId,
    reason: &String,
    created_at: &NaiveDateTime,
) -> CreateSelectMenuOption {
    let moderator = moderator_id
        .to_user(ctx)
        .await
        .expect("Failed to fetch moderator by its ID");
    let moderator_name = &moderator.name;

    let simple_created_at = format!("{}", created_at.format("%b %d, %Y"));

    build_select_menu_option_with_emoji(
        format!("@{moderator_name} - {simple_created_at}"),
        format!("{uuid}"),
        reason.trim(),
        "⚠️",
    )
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
