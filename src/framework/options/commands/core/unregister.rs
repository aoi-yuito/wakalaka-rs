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

use serenity::all::Command;
use tracing::error;

use crate::{
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    category = "Core",
    owners_only,
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Unegister command(s) for yours truly.
pub async fn unregister(
    ctx: Context<'_>,
    #[description = "Whether or not the command(s) should be global."]
    #[flag]
    global: bool,
) -> Result<(), Error> {
    let guild_id = models::guilds::guild_id(ctx).await;

    if global {
        let mut reply = messages::reply("Unregistering command(s) globally...", true);
        let reply_handle = ctx.send(reply).await?;

        let global_commands = Command::set_global_commands(ctx, vec![]).await;
        if let Err(why) = global_commands {
            error!("Couldn't set global commands: {why:?}");
            return Err(why.into());
        }

        reply = messages::ok_reply("I've unregistered every command globally.", true);
        reply_handle.edit(ctx, reply).await?;

        return Ok(());
    }

    let mut reply = messages::reply("Unregistering command(s)...", true);
    let reply_handle = ctx.send(reply).await?;

    let commands = guild_id.set_commands(ctx, vec![]).await;
    if let Err(why) = commands {
        error!("Couldn't set commands: {why:?}");
        return Err(why.into());
    }

    reply = messages::ok_reply("I've unregistered every command.", true);
    reply_handle.edit(ctx, reply).await?;

    Ok(())
}
