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
/// Register command(s) for yours truly.
pub async fn register(
    ctx: Context<'_>,
    #[description = "Whether or not the command(s) should be global."]
    #[flag]
    global: bool,
) -> Result<(), Error> {
    let guild_id = models::guilds::guild_id(ctx)?;

    let commands = &ctx.framework().options().commands;
    let commands_builder = create_application_commands(&commands);

    let command_count = commands_builder.len();

    if global {
        let mut reply = messages::reply("Registering global command(s)...", true);
        let reply_handle = ctx.send(reply).await?;

        let global_commands = Command::set_global_commands(ctx, commands_builder).await;
        if let Err(why) = global_commands {
            error!("Couldn't set global commands: {why:?}");
            return Err(why.into());
        }

        reply = messages::ok_reply(
            format!("Registered {command_count} global command(s)."),
            true,
        );
        reply_handle.edit(ctx, reply).await?;

        return Ok(());
    }

    let mut reply = messages::reply("Registering command(s)...", true);
    let reply_handle = ctx.send(reply).await?;

    let commands = guild_id.set_commands(ctx, commands_builder).await;
    if let Err(why) = commands {
        error!("Couldn't set commands: {why:?}");
        return Err(why.into());
    }

    reply = messages::ok_reply(format!("Registered {command_count} command(s)."), true);
    reply_handle.edit(ctx, reply).await?;

    Ok(())
}
