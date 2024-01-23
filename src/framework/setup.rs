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

use tracing::{error, info, warn};

use crate::framework::options::commands;
use crate::utility::models;
use crate::{serenity::Context, Data, Error};

pub async fn handle(ctx: &Context, data: Data) -> Result<Data, Error> {
    register_guild_commands(ctx).await;

    Ok(data)
}

async fn register_guild_commands(ctx: &Context) {
    let guild_id = models::guilds::guild_id_raw(ctx).await;
    let guild_name = models::guilds::guild_name_raw(ctx, guild_id).await;
    let guild_commands = commands::guild_commands().await;

    let number_of_guild_commands = guild_commands.len();
    if number_of_guild_commands == 0 {
        warn!("No guild command(s) to register in {guild_name}");
        return;
    }

    match poise::builtins::register_in_guild(&ctx.http, &guild_commands, guild_id).await {
        Ok(_) => {
            info!("Registered {number_of_guild_commands} guild command(s) in {guild_name}");
        }
        Err(why) => {
            error!("Couldn't register guild commands: {why:?}");
            return;
        }
    }
}
