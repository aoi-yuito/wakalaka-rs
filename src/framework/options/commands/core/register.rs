// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

use poise::samples::create_application_commands;
use serenity::all::Command;
use tracing::error;

use crate::{
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    owners_only,
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Register commands for yours truly.
pub async fn register(
    ctx: Context<'_>,
    #[description = "Whether or not the commands should be global."]
    #[flag]
    global: bool,
) -> Result<(), Error> {
    let guild_id = models::guilds::guild_id(ctx)?;

    let commands = &ctx.framework().options().commands;
    let commands_builder = create_application_commands(&commands);

    let command_count = commands_builder.len();

    if global {
        let mut reply = if command_count == 1 {
            messages::reply(None, "Registering global command...", true)
        } else {
            messages::reply(None, format!("Registering global commands..."), true)
        };

        let reply_handle = ctx.send(reply).await?;

        let global_commands = Command::set_global_commands(ctx, commands_builder).await;
        if let Err(why) = global_commands {
            error!("Failed to set global commands: {why:?}");
            return Err(why.into());
        }

        reply = if command_count == 1 {
            messages::ok_reply(None, "Registered `1` global command.", true)
        } else {
            messages::ok_reply(
                None,
                format!("Registered `{command_count}` global commands."),
                true,
            )
        };
        reply_handle.edit(ctx, reply).await?;

        return Ok(());
    }

    let mut reply = if command_count == 1 {
        messages::reply(None, "Registering command...", true)
    } else {
        messages::reply(None, format!("Registering commands..."), true)
    };

    let reply_handle = ctx.send(reply).await?;

    let commands = guild_id.set_commands(ctx, commands_builder).await;
    if let Err(why) = commands {
        error!("Failed to set commands: {why:?}");
        return Err(why.into());
    }

    reply = if command_count == 1 {
        messages::ok_reply(None, "Registered `1` command.", true)
    } else {
        messages::ok_reply(
            None,
            format!("Registered `{command_count}` commands."),
            true,
        )
    };
    reply_handle.edit(ctx, reply).await?;

    Ok(())
}
