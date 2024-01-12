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
    let (guild_id, guild_owner_id, guild_owner_locale, guild_preferred_locale) = (
        i64::from(guild.id),
        i64::from(guild.owner_id),
        match guild.owner_id.to_user(&ctx.http).await {
            Ok(user) => user.locale,
            Err(why) => {
                error!("Couldn't get old guild owner's locale: {why:?}");
                return;
            }
        },
        guild.preferred_locale.clone(),
    );

    let (new_guild_id, new_guild_owner_id, new_guild_owner_locale, new_guild_preferred_locale) = (
        i64::from(new_guild.id),
        i64::from(new_guild.owner_id),
        match new_guild.owner_id.to_user(&ctx.http).await {
            Ok(user) => user.locale,
            Err(why) => {
                error!("Couldn't get new guild owner's locale: {why:?}");
                return;
            }
        },
        new_guild.preferred_locale.clone(),
    );
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

    check_mismatch(
        guild_id,
        new_guild_id,
        guild_owner_id,
        new_guild_owner_id,
        guild_owner_locale,
        new_guild_owner_locale.clone(),
        guild_preferred_locale,
        new_guild_preferred_locale.clone(),
    );

    guilds::update_users(new_guild_owner_id, new_guild_owner_locale, database).await;
    guilds::update_guilds(
        new_guild_id,
        new_guild_owner_id,
        new_guild_preferred_locale,
        database,
    )
    .await;
    guilds::update_channels(new_guild_id, new_guild_channels, database).await;
}

fn check_mismatch(
    id: i64,
    new_id: i64,
    owner_id: i64,
    new_owner_id: i64,
    locale: Option<String>,
    new_locale: Option<String>,
    preferred_locale: String,
    new_preferred_locale: String,
) {
    if id != new_id {
        warn!("Mismatched guild ID(s)");
    }
    if owner_id != new_owner_id {
        warn!("Mismatched guild owner ID(s)");
    }
    if locale != new_locale {
        warn!("Mismatched guild owner locale(s)");
    }
    if preferred_locale != new_preferred_locale {
        warn!("Mismatched guild preferred locale(s)");
    }
}
