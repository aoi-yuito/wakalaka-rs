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

pub mod cache_ready;
pub mod interaction_create;
pub mod ready;

use serenity::{
    all::{ GuildId, Interaction, Ready, Command },
    async_trait,
    builder::CreateCommand,
    client::EventHandler,
};
use tracing::{ log::error, log::info };

use crate::{ Context, commands };

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::handle(ctx, interaction).await;
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        cache_ready::handle(ctx, guilds).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::handle(ctx, ready).await;
    }
}

async fn register_commands(
    ctx: &Context,
    guild_id: &GuildId,
    guild_name: &String,
    guild_commands: Vec<Command>,
    global_commands: Vec<Command>
) {
    let (guild_command_count, global_command_count) = (guild_commands.len(), global_commands.len());
    if guild_command_count == 0 {
        error!("No guild commands found in {guild_name}");
    } else if global_command_count == 0 {
        error!("No global commands found in {guild_name}");
    }

    register_guild_commands(&ctx, guild_id, &guild_name).await;
    register_global_commands(&ctx, &guild_name).await;
}

async fn register_global_commands(ctx: &Context, guild_name: &String) {
    let global_commands = registered_global_commands();
    let global_commands_count = global_commands.len();
    Command::set_global_commands(&ctx.http, global_commands).await.expect(
        "Expected global commands, but didn't find any"
    );

    info!("Registered {global_commands_count} global command(s) in {guild_name}");
}

async fn register_guild_commands(ctx: &Context, guild_id: &GuildId, guild_name: &String) {
    let commands = registered_guild_commands();
    let command_count = commands.len();
    guild_id
        .set_commands(&ctx.http, commands).await
        .expect("Expected guild commands, but didn't find any");

    info!("Registered {command_count} guild command(s) in {guild_name}");
}

fn registered_guild_commands() -> Vec<CreateCommand> {
    vec![
        commands::core::reload::register(),
        commands::core::restart::register(),
        commands::core::shutdown::register(),
        commands::misc::suggest::register(),
        commands::moderation::purge::register(),
        commands::web::booru::aibooru::register(),
        commands::web::booru::danbooru::register()
    ]
}

fn registered_global_commands() -> Vec<CreateCommand> {
    vec![commands::core::bot::register(), commands::misc::avatar::register()]
}
