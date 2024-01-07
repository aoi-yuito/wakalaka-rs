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
    all::{ colours::branding, CommandInteraction, UserId },
    builder::{ CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage },
};

use crate::Context;

pub(super) async fn user(ctx: &Context, interaction: &CommandInteraction) -> Option<String> {
    let user_id = user_id(interaction);

    let user = user_id.to_user(&ctx.http).await.expect("Expected user, but didn't find one");
    let user_avatar_url = user.avatar_url().unwrap_or_else(|| user.default_avatar_url());
    let user_name = user.name;

    let embed = CreateEmbed::default()
        .title(format!("{user_name}'s avatar"))
        .image(user_avatar_url)
        .color(branding::BLURPLE);

    let response_message = CreateInteractionResponseMessage::default().add_embed(embed);
    let response = CreateInteractionResponse::Message(response_message);

    match interaction.create_response(&ctx.http, response).await {
        Ok(_) => None,
        Err(why) => Some(format!("Error while creating response: {why}")),
    }
}

fn user_id(interaction: &CommandInteraction) -> UserId {
    let current_user_id = interaction.user.id;

    interaction.data.options
        .get(0)
        .and_then(|option| Some(option.value.clone()))
        .and_then(|value| value.as_user_id())
        .unwrap_or(current_user_id)
}
