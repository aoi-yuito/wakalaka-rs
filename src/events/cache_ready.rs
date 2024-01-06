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

use crate::events::*;
use crate::Context;
use serenity::all::Command;
use serenity::all::GuildId;

use tracing::{ log::error, log::info };

pub async fn handle(ctx: Context, guilds: Vec<GuildId>) {
    let guild_count = guilds.len();
    info!("Prepared cache for {guild_count} guild(s)");

    let guild_ids = &ctx.cache.guilds();
    for guild_id in guild_ids {
        let guild_name = {
            let guild = &ctx.cache.guild(guild_id).unwrap_or_else(|| {
                error!("No guild found");
                panic!("Error while retrieving guild");
            });
            guild.name.clone()
        };
        info!("Connected to {guild_name}");

        let (existing_guild_commands, existing_global_commands) = (
            guild_id.get_commands(&ctx.http).await.unwrap_or_else(|why| {
                error!("Error while retrieving existing guild commands: {why}");
                panic!("{why:?}");
            }),
            Command::get_global_commands(&ctx.http),
        );

        update_commands(
            &ctx,
            guild_id,
            &guild_name,
            existing_guild_commands,
            existing_global_commands.await.unwrap()
        ).await;
    }
}

async fn update_commands(
    ctx: &Context,
    guild_id: &GuildId,
    guild_name: &String,
    guild_commands: Vec<Command>,
    global_commands: Vec<Command>
) {
    let (guild_command_count, global_command_count) = (guild_commands.len(), global_commands.len());
    if guild_command_count == 0 {
        error!("No guild command(s) found in {guild_name}");
    } else if global_command_count == 0 {
        error!("No global command(s) found in {guild_name}");
    }

    let existing_guild_commands = guild_id.get_commands(&ctx.http).await.unwrap_or_else(|why| {
        error!("Error while retrieving existing guild command(s): {why}");
        panic!("{why:?}");
    });

    let existing_guild_command_names: Vec<String> = existing_guild_commands
        .iter()
        .map(|cmd| cmd.name.clone())
        .collect();

    let existing_commands = guild_commands.iter().chain(global_commands.iter());
    for existing_command in existing_commands {
        ();
        let existing_command_name = &existing_command.name;
        if existing_guild_command_names.contains(existing_command_name) {
            let existing_command_id = &existing_command.id;

            guild_id.delete_command(&ctx.http, *existing_command_id).await.unwrap_or_else(|why| {
                error!("Error while deleting guild command(s): {why:?}");
                panic!("{why:?}");
            });
            info!("Deleted {existing_command_name:?} from {guild_name}");
        }
    }

    add_guild_commands(&ctx, guild_id, &guild_name).await;
    add_global_commands(&ctx, &guild_name).await;

    let existing_command_count = existing_guild_commands.len();
    info!("Updated {existing_command_count} guild command(s) in {guild_name}");
}

async fn add_global_commands(ctx: &Context, guild_name: &String) {
    let global_commands = created_global_commands();
    let global_commands_count = global_commands.len();
    for global_command in &global_commands {
        Command::create_global_command(&ctx.http, global_command.clone()).await.unwrap_or_else(
            |why| {
                error!("Error while registering global command(s): {why:?}");
                panic!("{why:?}");
            }
        );
    }

    info!("Registered {global_commands_count} global command(s) in {guild_name}");
}

async fn add_guild_commands(ctx: &Context, guild_id: &GuildId, guild_name: &String) {
    let commands = created_guild_commands();
    let command_count = created_guild_commands().len();
    guild_id.set_commands(&ctx.http, commands).await.unwrap_or_else(|why| {
        error!("Error while registering guild command(s): {why:?}");
        panic!("{why:?}");
    });

    info!("Registered {command_count} guild command(s) in {guild_name}");
}
