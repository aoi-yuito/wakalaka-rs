// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod announce;
mod restrict;
mod unrestrict;

use poise::Command;

use crate::{Data, Error};

pub(super) async fn commands() -> Vec<Command<Data, Error>> {
    vec![
        restrict::restrict(),
        unrestrict::unrestrict(),
        announce::announce(),
    ]
}
