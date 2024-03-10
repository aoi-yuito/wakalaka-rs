// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod core;
mod fun;
mod info;
mod integrations;
mod manager;
mod misc;
mod moderator;
mod music;

use poise::Command;

use crate::{Data, Error};

pub(crate) async fn commands() -> Vec<Command<Data, Error>> {
    let mut commands = vec![];
    commands.extend(core::commands().await);
    commands.extend(fun::commands().await);
    commands.extend(info::commands().await);
    commands.extend(integrations::commands().await);
    commands.extend(manager::commands().await);
    commands.extend(misc::commands().await);
    commands.extend(moderator::commands().await);
    commands.extend(music::commands().await);
    commands
}
