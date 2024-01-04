use serenity::{
    all::{GuildId, Message, UserId},
    client::Cache,
};
use tracing::warn;

use crate::Context;

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
pub mod core;

use tracing::log::error;

pub async fn is_owner_of_guild(ctx: &Context) -> bool {
    let cloned_cache = ctx.cache.clone();

    let guild_ids = cloned_cache.guilds();
    for guild_id in guild_ids {
        let http = &ctx.http;

        let guild = guild_id.to_partial_guild(http).await.unwrap_or_else(|why| {
            error!("An error occurred while retrieving guild: {why}");

            panic!();
        });

        let guild_owner_id = guild.owner_id;

        let guild_members = guild_id
            .members(http, None, None)
            .await
            .unwrap_or_else(|why| {
                error!("An error occurred while retrieving guild members: {why}");

                panic!();
            });
        for guild_member in guild_members {
            let user_bot = guild_member.user.bot;
            if user_bot {
                continue;
            }

            if guild_member.user.id.eq(&guild_owner_id) {
                true;
            } else {
                false;
            }
        }
    }

    false
}
