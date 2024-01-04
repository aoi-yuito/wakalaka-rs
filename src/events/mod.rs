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

use serenity::{
    all::{CommandInteraction, GuildId, Interaction, Ready},
    async_trait,
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage},
    client::EventHandler,
};
use tracing::{log::error, log::info};

use crate::commands::*;
use crate::Context;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let command_user = &command.user.name;
            let command_name = &command.data.name;
            let channel_name = &command.channel_id.name(&ctx).await.unwrap_or_else(|why| {
                error!("{why}");
                panic!("Error while retrieving channel name");
            });
            info!("@{command_user} executed {command_name:?} in #{channel_name}");

            let content = register_command(&ctx, &command).await;
            register_slash_commands(&ctx, &command, content).await;
        }
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let guild_count = guilds.len();
        info!("Prepared cache for {guild_count} guild(s)");

        for guild in guilds {
            let guild_name = guild.name(&ctx).unwrap_or_else(|| {
                error!("No guild name found");
                panic!("Error while retrieving guild name");
            });
            info!("\t{guild_name}");
        }

        let cache = &ctx.cache;

        let guild_ids = cache.guilds();
        for guild_id in guild_ids {
            let (guild_name, guild_member_count, guild_role_count, guild_channel_count) = {
                let guild = cache.guild(guild_id).unwrap_or_else(|| {
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

            let registered_commands = guild_id.set_commands(&ctx.http, created_commands()).await;
            if let Ok(registered_commands) = registered_commands {
                let registered_command_count = &registered_commands.len();
                info!("Registered {registered_command_count} command(s) in {guild_name}");

                for registered_command in registered_commands {
                    let registered_command_name = &registered_command.name;
                    let registered_command_description = &registered_command.description;
                    info!("\t{registered_command_name:?} - {registered_command_description}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let user_name = &ready.user.name;
        info!("Logged in as @{user_name}");

        let cache = &ctx.cache;
    }
}

fn created_commands() -> Vec<CreateCommand> {
    vec![core::restart::register()]
}

async fn register_slash_commands(
    ctx: &Context,
    command: &CommandInteraction,
    content: Option<String>,
) {
    if let Some(content) = content {
        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = command.create_response(&ctx.http, builder).await {
            error!("{why}")
        }
    }
}

async fn register_command(ctx: &Context, command: &CommandInteraction) -> Option<String> {
    let command_data_name = &command.data.name;
    let command_data_options = &command.data.options();

    match command_data_name.as_str() {
        "restart" => Some(core::restart::run(&ctx, command, command_data_options).await),
        _ => Some("Unknown command.".to_string()),
    }
}
