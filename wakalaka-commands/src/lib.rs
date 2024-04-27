// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod core;
pub mod fun;
pub mod info;
pub mod manager;
pub mod moderator;

use serenity::all::User;
use wakalaka_core::{
    builders,
    types::{Command, Context, Throwable},
};

pub async fn gather_all_commands() -> Vec<Command> {
    let mut commands = vec![];
    commands.extend(core::commands().await);
    commands.extend(fun::commands().await);
    commands.extend(info::commands().await);
    commands.extend(manager::commands().await);
    commands.extend(moderator::commands().await);
    commands
}

async fn is_user_bot_or_system(ctx: Context<'_>, user: &User) -> Throwable<bool> {
    if user.bot {
        let reply = builders::replies::build_error_reply_with_embed("Cannot act on a bot.", true);

        ctx.send(reply).await?;

        return Ok(true);
    } else if user.system {
        let reply =
            builders::replies::build_error_reply_with_embed("Cannot act on a system user.", true);

        ctx.send(reply).await?;

        return Ok(true);
    }

    Ok(false)
}
