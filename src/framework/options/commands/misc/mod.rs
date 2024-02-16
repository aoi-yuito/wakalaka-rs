// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod avatar;
mod banner;

use poise::Command;

use crate::{Data, Error};

pub(super) async fn commands() -> Vec<Command<Data, Error>> {
    vec![avatar::avatar(), banner::banner()]
}
