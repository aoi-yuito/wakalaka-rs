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

use poise::serenity_prelude::Context;
use serenity::all::{Guild, GuildChannel};
use tracing::error;

use crate::{database::guilds, Data};

pub(crate) async fn handle(guild: &Guild, is_new: bool, ctx: &Context, data: &Data) {
    let pool = &data.pool;

    let (guild_id, guild_owner_id, guild_preferred_locale) = (
        i64::from(guild.id),
        i64::from(guild.owner_id),
        guild.preferred_locale.clone(),
    );

    let users = match guild.members(&ctx.http, None, None).await {
        Ok(users) => users,
        Err(why) => {
            error!("Couldn't get guild members: {why:?}");
            return;
        }
    };

    let channels = match guild.channels(&ctx.http).await {
        Ok(channels) => channels,
        Err(why) => {
            error!("Couldn't get guild channels: {why:?}");
            return;
        }
    };
    let guild_channels = channels
        .into_iter()
        .map(|(_, channel)| channel)
        .collect::<Vec<GuildChannel>>();

    if is_new {
        guilds::insert_users(users, pool).await;
        guilds::insert_guilds(guild_id, guild_owner_id, guild_preferred_locale, pool).await;
        guilds::insert_channels(guild_id, guild_channels, pool).await;
    } else {
        guilds::update_users(users, pool).await;
        guilds::update_guilds(guild_id, guild_owner_id, guild_preferred_locale, pool).await;
        guilds::update_channels(guild_id, guild_channels, pool).await;
    }
}
