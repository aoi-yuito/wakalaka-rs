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

use serenity::all::{Guild, GuildChannel, PartialGuild};

use poise::serenity_prelude::Context;
use tracing::{error, warn};

use crate::{database::guilds, Data};

pub(crate) async fn handle(
    guild: &Option<Guild>,
    new_guild: &PartialGuild,
    ctx: &Context,
    data: &Data,
) {
    let database = &data.pool;

    let guild = match guild {
        Some(guild) => guild,
        None => {
            error!("Couldn't get old guild");
            return;
        }
    };

    let (new_guild_id, new_guild_owner_id, new_guild_preferred_locale) = (
        i64::from(new_guild.id),
        i64::from(new_guild.owner_id),
        new_guild.preferred_locale.clone(),
    );

    let guild_users = match guild.members(&ctx.http, None, None).await {
        Ok(users) => users,
        Err(why) => {
            error!("Couldn't get old guild members: {why:?}");
            return;
        }
    };

    let new_channels = match new_guild.channels(&ctx.http).await {
        Ok(channels) => channels,
        Err(why) => {
            error!("Couldn't get new guild channels: {why:?}");
            return;
        }
    };
    let new_guild_channels = new_channels
        .into_iter()
        .map(|(_, channel)| channel)
        .collect::<Vec<GuildChannel>>();

    guilds::update_users(guild_users, database).await;
    guilds::update_guilds(
        new_guild_id,
        new_guild_owner_id,
        new_guild_preferred_locale,
        database,
    )
    .await;
    guilds::update_channels(new_guild_id, new_guild_channels, database).await;
}
