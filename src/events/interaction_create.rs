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
    all::{CommandInteraction, Interaction},
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
};

use crate::events::*;
use crate::Context;
use tracing::{log::error, log::info, log::warn};

pub async fn handle(ctx: Context, interaction: Interaction) {
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

async fn register_slash_commands(
    ctx: &Context,
    command: &CommandInteraction,
    content: Option<String>,
) {
    if let Some(content) = content {
        let response_message = CreateInteractionResponseMessage::new().content(content);
        let response = CreateInteractionResponse::Message(response_message);

        if let Err(why) = command.create_response(&ctx.http, response).await {
            error!("{why}")
        }
    }
}

async fn register_command(ctx: &Context, command: &CommandInteraction) -> Option<String> {
    let command_options = &command.data.options();

    let command_name = &command.data.name;
    match command_name.as_str() {
        "avatar" => Some(general::avatar::run(&ctx, command).await?),
        "restart" => Some(core::restart::run(&ctx, command, command_options).await?),
        _ => {
            warn!("{command_name:?} isn't implemented yet");
            None
        }
    }
}
