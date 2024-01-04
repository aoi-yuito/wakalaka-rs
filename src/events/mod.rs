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
    all::{Interaction, Ready},
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::EventHandler,
};
use tracing::{error, log::info};

use crate::commands::*;
use crate::Context;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let command_user_id = &command.user.id;
            let command_name = &command.data.name;
            let command_channel_id = &command.channel_id;
            info!("{command_user_id} invoked the '{command_name}' command in {command_channel_id}");

            let command_data_name = &command.data.name;
            let command_data_options = &command.data.options();

            let content = match command_data_name.as_str() {
                "restart" => Some(core::restart::run(&ctx, command_data_options)),
                _ => Some(format!("{command_data_name} isn't a known command")),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    error!("An error occurred while responding to command: {why}")
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let user_id = &ready.user.id;
        info!("Logged in as {user_id}");

        let guild_ids = ctx.cache.guilds();
        for guild_id in guild_ids {
            info!("Connected to {guild_id}");

            let commands = guild_id
                .set_commands(&ctx.http, vec![core::restart::register()])
                .await;
            if let Ok(command) = commands {
                let command_count = &command.len();
                info!("Registered {command_count} command(s) in {guild_id}");
            }

            // if you want to make globals, use "Command::create_global_command"
        }
    }
}
