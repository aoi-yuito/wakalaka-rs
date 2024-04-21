// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateSelectMenuOption, ReactionType};

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
