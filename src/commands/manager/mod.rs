// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod emojis;
mod nick;
mod purge;
mod roles;
mod slowmode;
mod stickers;

use wakalaka_core::types::Command;

pub async fn commands() -> Vec<Command> {
    vec![
        emojis::emojis(),
        purge::purge(),
        nick::nick(),
        roles::roles(),
        stickers::stickers(),
        slowmode::slowmode(),
    ]
}