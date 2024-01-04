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
    all::{Command, CommandInteraction, Interaction, Ready},
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
            let channel_name = &command.channel_id.name(&ctx).await.unwrap();
            info!("@{command_user} executed {command_name:?} in #{channel_name}");

            let content = register_command(&ctx, &command).await;
            register_slash_commands(&ctx, &command, content).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let user_name = &ready.user.name;
        info!("Logged in as @{user_name}");

        let cache = &ctx.cache;

        let guild_ids = cache.guilds();
        for guild_id in guild_ids {
            let mut guild_name = String::new();
            let guild_members = guild_id.members(&ctx.http, None, None).await.unwrap().len();
            let guild_roles = guild_id.roles(&ctx.http).await.unwrap().len();
            let guild_channels = guild_id.channels(&ctx.http).await.unwrap().len();

            let partial_guild = guild_id.to_partial_guild(&ctx.http).await;
            if let Ok(guild) = partial_guild {
                guild_name = guild.name;
            }

            info!("Connected to {guild_name}");
            info!("\t{guild_name} has {guild_members} members");
            info!("\t{guild_name} has {guild_roles} roles");
            info!("\t{guild_name} has {guild_channels} channels");

            let registered_commands = guild_id.set_commands(&ctx.http, created_commands()).await;
            if let Ok(registered_command) = registered_commands {
                let commands = &registered_command.len();
                info!("Registered {commands} command(s) in {guild_name}");
            }

            // if you want to make globals, use "Command::create_global_command"
        }
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
            error!("An error occurred while responding to command: {why}")
        }
    }
}

async fn register_command(ctx: &Context, command: &CommandInteraction) -> Option<String> {
    let command_data_name = &command.data.name;
    let command_data_options = &command.data.options();

    match command_data_name.as_str() {
        "restart" => Some(core::restart::run(&ctx, command_data_options).await),
        _ => Some("Unknown command.".to_string()),
    }
}
