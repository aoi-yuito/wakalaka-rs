// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod about;
mod invite;

use wakalaka_core::types::Command;

pub async fn commands() -> Vec<Command> {
    vec![about::about(), invite::invite()]
}
