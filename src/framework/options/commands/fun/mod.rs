// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::Command;

use crate::{Data, Error};

mod eightball;
mod flip;
mod hug;
mod roll;

pub(super) async fn commands() -> Vec<Command<Data, Error>> {
    vec![
        eightball::eightball(),
        flip::flip(),
        hug::hug(),
        roll::roll(),
    ]
}
