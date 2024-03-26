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

use poise::Command;
use serenity::all::GuildId;
use tracing::{error, info, warn};

use crate::{Data, Error, SContext, Throwable};

pub(crate) async fn register_guild_commands(ctx: &SContext, guild_id: &GuildId) -> Throwable<()> {
    let commands = commands().await;

    let command_count = commands.len();
    if command_count == 0 {
        warn!("No commands to register");

        return Ok(());
    }

    let message = if command_count < 1 {
        format!("Registered {command_count} command")
    } else {
        format!("Registered {command_count} commands")
    };

    if let Err(why) = poise::builtins::register_in_guild(ctx, &commands, *guild_id).await {
        error!("Failed to register in guild: {why}");
        return Err(why.into());
    }

    info!("{message}");

    Ok(())
}

pub(crate) async fn commands() -> Vec<Command<Data, Error>> {
    let mut commands = vec![];
    commands.extend(core::commands().await);
    commands.extend(fun::commands().await);
    commands.extend(info::commands().await);
    // commands.extend(integrations::commands().await);
    commands.extend(manager::commands().await);
    commands.extend(misc::commands().await);
    commands.extend(moderator::commands().await);
    commands
}
