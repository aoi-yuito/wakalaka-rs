// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod ban;
mod kick;
mod timeout;
mod unban;
mod unwarn;
mod warn;

use poise::Command;

use crate::{Data, Error};

pub(super) async fn commands() -> Vec<Command<Data, Error>> {
    vec![
        ban::ban(),
        kick::kick(),
        timeout::timeout(),
        unban::unban(),
        unwarn::unwarn(),
        warn::warn(),
    ]
}
