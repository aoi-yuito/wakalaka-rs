// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod core;
pub mod fun;
pub mod info;
pub mod manager;
pub mod moderator;

use serenity::all::{GuildId, User};
use wakalaka_core::types::{Command, Context, SContext, Throwable};
use wakalaka_utils::builders;

pub(crate) async fn register_guild_commands(ctx: &SContext, guild_id: &GuildId) -> Throwable<()> {
    let cmds = gather_all_commands().await;

    let cmd_count = cmds.len();
    if cmd_count > 0 {
        poise::builtins::register_in_guild(ctx, &cmds, *guild_id).await?;
    }

    Ok(())
}

pub(crate) async fn gather_all_commands() -> Vec<Command> {
    let mut commands = vec![];
    commands.extend(core::commands().await);
    commands.extend(fun::commands().await);
    commands.extend(info::commands().await);
    commands.extend(manager::commands().await);
    commands
}

async fn is_user_bot_or_system(ctx: Context<'_>, user: &User) -> Throwable<bool> {
    if user.bot {
        let reply = builders::replies::build_error_reply_with_embed(
            "Cannot restrict bots from using yours truly.",
            true,
        );

        ctx.send(reply).await?;

        return Ok(true);
    } else if user.system {
        let reply = builders::replies::build_error_reply_with_embed(
            "Cannot restrict system users from using yours truly.",
            true,
        );

        ctx.send(reply).await?;

        return Ok(true);
    }

    Ok(false)
}
