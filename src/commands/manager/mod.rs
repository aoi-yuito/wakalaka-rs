// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod nick;
mod slowmode;

use wakalaka_core::types::Command;

pub async fn commands() -> Vec<Command> {
    vec![nick::nick(), slowmode::slowmode()]
}
