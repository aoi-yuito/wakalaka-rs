// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ButtonStyle, CreateButton, ReactionType};

pub(crate) const BUTTON_PAGINATE_FIRST: &str = "pgn_first";
pub(crate) const BUTTON_PAGINATE_PREVIOUS: &str = "pgn_prev";
pub(crate) const BUTTON_PAGINATE_NEXT: &str = "pgn_next";
pub(crate) const BUTTON_PAGINATE_LAST: &str = "pgn_last";

pub(crate) fn pagination_buttons(disabled: (bool, bool, bool, bool)) -> Vec<CreateButton> {
    vec![
        CreateButton::new(BUTTON_PAGINATE_FIRST)
            .style(ButtonStyle::Primary)
            .emoji(ReactionType::Unicode(format!("⏮️")))
            .disabled(disabled.0),
        CreateButton::new(BUTTON_PAGINATE_PREVIOUS)
            .style(ButtonStyle::Primary)
            .emoji(ReactionType::Unicode(format!("⬅️")))
            .disabled(disabled.1),
        CreateButton::new(BUTTON_PAGINATE_NEXT)
            .style(ButtonStyle::Primary)
            .emoji(ReactionType::Unicode(format!("➡️")))
            .disabled(disabled.2),
        CreateButton::new(BUTTON_PAGINATE_LAST)
            .style(ButtonStyle::Primary)
            .emoji(ReactionType::Unicode(format!("⏭️")))
            .disabled(disabled.3),
    ]
}
