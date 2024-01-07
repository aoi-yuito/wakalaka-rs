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

pub mod core;
pub mod moderation;
pub mod web;
pub mod misc;

use serenity::all::{ CommandDataOption, CommandInteraction, GuildId, Command };
use tracing::{ log::error, log::warn, log::info };

use crate::{ Context, events };

fn command_option(interaction: &CommandInteraction, index: usize) -> Option<&CommandDataOption> {
    if let Some(option) = interaction.data.options.get(index) { Some(option) } else { None }
}

pub(super) async fn register_global_commands(ctx: &Context, guild_name: &String) {
    let global_commands = events::registered_global_commands();
    let global_commands_count = global_commands.len();
    Command::set_global_commands(&ctx.http, global_commands).await.unwrap_or_else(|why| {
        error!("Error while registering global command(s): {why:?}");
        panic!("{why:?}");
    });

    info!("Registered {global_commands_count} global command(s) in {guild_name}");
}

pub(super) async fn register_guild_commands(
    ctx: &Context,
    guild_id: &GuildId,
    guild_name: &String
) {
    let commands = events::registered_guild_commands();
    let command_count = commands.len();
    guild_id.set_commands(&ctx.http, commands).await.unwrap_or_else(|why| {
        error!("Error while registering guild command(s): {why:?}");
        panic!("{why:?}");
    });

    info!("Registered {command_count} guild command(s) in {guild_name}");
}

async fn has_manage_messages_permission(ctx: &Context, interaction: &CommandInteraction) -> bool {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return false;
        }
    };

    let member = guild_id.member(&ctx.http, interaction.user.id).await.unwrap_or_else(|why| {
        error!("Error while retrieving guild member: {why}");
        panic!("{why:?}");
    });

    let permissions = member.permissions(&ctx.cache);
    if let Ok(permissions) = permissions {
        return permissions.manage_messages();
    }

    let user_name = &interaction.user.name;
    let command_name = &interaction.data.name;
    let channel_name = &interaction.channel_id.name(&ctx).await.unwrap_or_else(|why| {
        error!("Error while retrieving channel name: {why}");
        panic!("{why:?}");
    });
    warn!("@{user_name} doesn't have permission(s) to execute {command_name:?} in #{channel_name}");

    return false;
}

async fn has_administrator_permission(ctx: &Context, interaction: &CommandInteraction) -> bool {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return false;
        }
    };

    let member = guild_id.member(&ctx.http, interaction.user.id).await.unwrap_or_else(|why| {
        error!("Error while retrieving guild member: {why}");
        panic!("{why:?}");
    });

    let permissions = member.permissions(&ctx.cache);
    if let Ok(permissions) = permissions {
        return permissions.administrator();
    }

    let user_name = &interaction.user.name;
    let command_name = &interaction.data.name;
    let channel_name = &interaction.channel_id.name(&ctx).await.unwrap_or_else(|why| {
        error!("Error while retrieving channel name: {why}");
        panic!("{why:?}");
    });
    warn!("@{user_name} doesn't have permission(s) to execute {command_name:?} in #{channel_name}");

    return false;
}
