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
    all::{CommandInteraction, Interaction, Ready},
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
            let command_user = &command.user.global_name;
            let command_name = format!("/{}", &command.data.name);
            info!("{command_user:#?} invoked '{command_name}'");

            let content = register_command(&ctx, &command);
            register_slash_commands(&ctx, &command, content).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let user_name = &ready.user.name;
        info!("Logged in as {user_name}");

        let guild_ids = ctx.cache.guilds();
        for guild_id in guild_ids {
            let mut guild_name = String::new();

            let partial_guild = guild_id.to_partial_guild(&ctx.http).await;
            if let Ok(guild) = partial_guild {
                guild_name = guild.name;
            }

            info!("Connected to {guild_name}");

            let commands = guild_id
                .set_commands(&ctx.http, vec![core::restart::register()])
                .await;
            if let Ok(command) = commands {
                let command_count = &command.len();
                info!("Registered {command_count} command(s) in {guild_name}");
            }

            // if you want to make globals, use "Command::create_global_command"
        }
    }
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

fn register_command(ctx: &Context, command: &CommandInteraction) -> Option<String> {
    let command_data_name = &command.data.name;
    let command_data_options = &command.data.options();

    match command_data_name.as_str() {
        "restart" => Some(core::restart::run(&ctx, command_data_options)),
        _ => Some("Unknown command. Try /help for a list of commands".to_string()),
    }
}
