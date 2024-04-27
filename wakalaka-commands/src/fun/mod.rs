// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod eightball;
mod flip;
mod hug;
mod roll;

use wakalaka_core::types::Command;

pub async fn commands() -> Vec<Command> {
    vec![
        eightball::eightball(),
        flip::flip(),
        hug::hug(),
        roll::roll(),
    ]
}
