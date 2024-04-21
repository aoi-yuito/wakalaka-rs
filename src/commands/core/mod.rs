// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod restrict;
mod unrestrict;

use wakalaka_core::types::Command;

pub(super) async fn commands() -> Vec<Command> {
    vec![restrict::restrict(), unrestrict::unrestrict()]
}
