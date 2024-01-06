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

use crate::{commands, Context};
use serenity::{
    all::{colours::branding, CommandInteraction, CommandOptionType},
    builder::{
        CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Option<String> {
    let command = commands::command(interaction, 0);
    match command.name.as_str() {
        "user" => user(interaction, ctx).await,
        _ => None,
    }
}

async fn user(
    interaction: &CommandInteraction,
    ctx: &serenity::prelude::Context,
) -> Option<String> {
    let user_id = interaction
        .data
        .options
        .get(0)
        .and_then(|option| option.value.as_user_id())
        .expect("Error while getting user ID");
    let user = user_id
        .to_user(&ctx.http)
        .await
        .expect("Error while getting user from user ID");
    let user_avatar_url = user
        .avatar_url()
        .unwrap_or_else(|| user.default_avatar_url());
    let user_name = user.name;

    let embed = CreateEmbed::default()
        .title(format!("{user_name}'s avatar"))
        .image(user_avatar_url)
        .color(branding::BLURPLE);

    let response_message = CreateInteractionResponseMessage::default();

    let message = response_message.add_embed(embed);
    let response = CreateInteractionResponse::Message(message);

    match interaction.create_response(&ctx.http, response).await {
        Ok(_) => None,
        Err(why) => Some(format!("Error while creating response: {why}")),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("avatar")
        .description("Fetches user's avatar.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "ID (or mention) of user to fetch avatar from.",
            )
            .required(true),
        )
}
