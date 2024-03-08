// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Command;

use crate::{utils::components, Context, Throwable};

#[poise::command(
    prefix_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    required_bot_permissions = "SEND_MESSAGES",
    owners_only,
    guild_only,
    user_cooldown = 5
)]
/// Make commands unavailable.
pub(super) async fn unregister(
    ctx: Context<'_>,
    #[description = "Whether the commands should be globalised."]
    #[flag]
    global: bool,
) -> Throwable<()> {
    let guild_id = ctx.guild_id().unwrap();

    let commands = &ctx.framework().options().commands;

    let command_count = commands.len();

    if global {
        let mut global_message = if command_count == 1 {
            format!("Unregistering a command globally...")
        } else {
            format!("Unregistering {command_count} commands globally...")
        };

        let mut reply = components::replies::reply_embed(global_message, true);

        let reply_handle = ctx.send(reply).await?;

        Command::set_global_commands(ctx, vec![]).await?;

        global_message = if command_count == 1 {
            format!("Command has been unregistered globally.")
        } else {
            format!("{command_count} commands have been unregistered globally.")
        };

        reply = components::replies::ok_reply_embed(global_message, true);

        reply_handle.edit(ctx, reply).await?;
    } else {
        let mut message = if command_count == 1 {
            format!("Unregistering a command...")
        } else {
            format!("Unregistering {command_count} commands...")
        };

        let mut reply = components::replies::reply_embed(message, true);

        let reply_handle = ctx.send(reply).await?;

        guild_id.set_commands(ctx, vec![]).await?;

        message = if command_count == 1 {
            format!("Command has been unregistered.")
        } else {
            format!("{command_count} commands have been unregistered.")
        };

        reply = components::replies::ok_reply_embed(message, true);

        reply_handle.edit(ctx, reply).await?;
    }

    Ok(())
}
