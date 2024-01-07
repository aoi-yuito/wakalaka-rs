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
    all::{ CommandInteraction, Interaction },
    builder::{ CreateInteractionResponse, CreateInteractionResponseMessage },
};
use tracing::{ log::error, log::info, log::warn };

use crate::commands;
use crate::Context;

pub async fn handle(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        let command_user = &command.user.name;
        let command_name = &command.data.name;
        let channel_name = &command.channel_id.name(&ctx).await.unwrap_or_else(|why| {
            error!("Error while retrieving channel name: {why}");
            panic!("{why:?}");
        });
        info!("@{command_user} executed {command_name:?} in #{channel_name}");

        let content = command_content(&ctx, &command).await;
        register_command_response(&ctx, &command, content).await;
    }
}

async fn register_command_response(
    ctx: &Context,
    command: &CommandInteraction,
    content: Option<String>
) {
    if let Some(content) = content {
        let response_message = CreateInteractionResponseMessage::new().content(content);
        let response = CreateInteractionResponse::Message(response_message);

        if let Err(why) = command.create_response(&ctx.http, response).await {
            error!("{why:?}");
        }
    }
}

async fn command_content(ctx: &Context, interaction: &CommandInteraction) -> Option<String> {
    let command_options = &interaction.data.options();

    let command_name = &interaction.data.name;
    match command_name.as_str() {
        "aibooru" =>
            Some(commands::web::booru::aibooru::run(&ctx, interaction, command_options).await?),
        "avatar" => Some(commands::misc::avatar::run(&ctx, interaction).await?),
        "danbooru" =>
            Some(commands::web::booru::danbooru::run(&ctx, interaction, command_options).await?),
        "shutdown" => Some(commands::core::shutdown::run(&ctx, interaction).await?),
        "suggest" => Some(commands::misc::suggest::run(&ctx, interaction, command_options).await?),
        "purge" =>
            Some(commands::moderation::purge::run(&ctx, interaction, command_options).await?),
        "reload" => Some(commands::core::reload::run(&ctx, interaction, command_options).await?),
        "restart" => Some(commands::core::restart::run(&ctx, interaction, command_options).await?),
        _ => {
            warn!("{command_name:?} isn't implemented yet");
            None
        }
    }
}
