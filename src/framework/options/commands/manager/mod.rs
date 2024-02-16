// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod emojis;
mod nick;
mod purge;
mod roles;
mod slowmode;

use poise::Command;

use crate::{Data, Error};

pub(super) async fn commands() -> Vec<Command<Data, Error>> {
    vec![
        emojis::emojis(),
        nick::nick(),
        purge::purge(),
        roles::roles(),
        slowmode::slowmode(),
    ]
}
