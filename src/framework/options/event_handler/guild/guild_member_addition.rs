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
    all::{Member, Mentionable},
    builder::CreateMessage,
};
use tracing::{error, info};

use crate::{
    database::{guilds, users},
    utility::models,
    Data,
};

pub async fn handle(new_member: &Member, ctx: &crate::serenity::Context, data: &Data) {
    let pool = &data.pool;

    let guild_id = new_member.guild_id;
    let guild_name = models::guilds::guild_name_raw(ctx, guild_id).await;

    let members = match models::members::members_raw(&ctx, &guild_id).await {
        Ok(members) => members,
        Err(_) => {
            return;
        }
    };

    if let Err(why) = users::insert_into_users(&members, pool).await {
        error!("Failed to insert into Users: {why:?}");
    } else {
        let user = &new_member.user;
        let (user_name, user_mention) = (&user.name, user.mention());

        info!("@{user_name} joined {guild_name}");

        let welcome_channel_id = guilds::check_welcome_channel(&guild_id, pool).await;
        if let Some(channel_id) = welcome_channel_id {
            let message_builder = CreateMessage::default()
                .content(format!("Welcome to **{guild_name}**, {user_mention}!"));
            let message = channel_id.send_message(&ctx, message_builder).await;
            if let Err(why) = message {
                error!("Failed to send message: {why:?}");
            }
        }
    }
}
