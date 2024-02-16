// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod about;
mod invite;
mod lookup;

use poise::Command;

use crate::{Data, Error};

pub(super) async fn commands() -> Vec<Command<Data, Error>> {
    vec![lookup::lookup(), about::about(), invite::invite()]
}
