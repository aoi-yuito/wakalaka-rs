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

use serenity::all::GuildId;

use crate::events::*;
use crate::Context;
use tracing::{log::error, log::info};

pub async fn handle(ctx: Context, guilds: Vec<GuildId>) {
    let guild_count = guilds.len();
    info!("Prepared cache for {guild_count} guild(s)");

    for guild in guilds {
        let guild_name = guild.name(&ctx).unwrap_or_else(|| {
            error!("No guild name found");
            panic!("Error while retrieving guild name");
        });
        info!("\t{guild_name}");
    }

    let guild_ids = &ctx.cache.guilds();
    for guild_id in guild_ids {
        let (guild_name, guild_member_count, guild_role_count, guild_channel_count) = {
            let guild = &ctx.cache.guild(guild_id).unwrap_or_else(|| {
                error!("No guild found");
                panic!("Error while retrieving guild");
            });

            (
                guild.name.clone(),
                guild.members.len(),
                guild.roles.len(),
                guild.channels.len(),
            )
        };
        info!("Connected to {guild_name}");
        info!("\t{guild_name} has {guild_member_count} members");
        info!("\t{guild_name} has {guild_role_count} roles");
        info!("\t{guild_name} has {guild_channel_count} channels");

        register_commands(&ctx, *guild_id, guild_name).await;
    }
}

async fn register_commands(ctx: &Context, guild_id: GuildId, guild_name: String) {
    let commands = guild_id.set_commands(&ctx.http, created_commands()).await;
    if let Ok(commands) = commands {
        let command_count = &commands.len();
        info!("Registered {command_count} command(s) in {guild_name}");

        for command in commands {
            let command_name = &command.name;
            let command_description = &command.description;
            info!("\t{command_name:?} - {command_description}");
        }
    } else {
        panic!("Error while registering command(s)");
    }
}
