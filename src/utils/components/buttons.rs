// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ButtonStyle, CreateButton, ReactionType};

pub(crate) fn first_page_button(disabled: bool) -> CreateButton {
    CreateButton::new("first_page")
        .style(ButtonStyle::Primary)
        .emoji(ReactionType::Unicode(format!("⏮️")))
        .disabled(disabled)
}

pub(crate) fn previous_page_button(disabled: bool) -> CreateButton {
    CreateButton::new("previous_page")
        .style(ButtonStyle::Primary)
        .emoji(ReactionType::Unicode(format!("⬅️")))
        .disabled(disabled)
}

pub(crate) fn next_page_button(disabled: bool) -> CreateButton {
    CreateButton::new("next_page")
        .style(ButtonStyle::Primary)
        .emoji(ReactionType::Unicode(format!("➡️")))
        .disabled(disabled)
}

pub(crate) fn last_page_button(disabled: bool) -> CreateButton {
    CreateButton::new("last_page")
        .style(ButtonStyle::Primary)
        .emoji(ReactionType::Unicode(format!("⏭️")))
        .disabled(disabled)
}
