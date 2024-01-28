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

use serenity::all::{Guild, GuildId, UserId};
use tracing::warn;

use crate::Context;

pub async fn owner_id(ctx: Context<'_>) -> UserId {
    guild(ctx).await.owner_id
}

pub async fn guild_name_from_guild_id_raw(
    ctx: &crate::serenity::Context,
    guild_id: GuildId,
) -> String {
    if let Some(value) = guild_id.name(ctx) {
        value
    } else {
        warn!("Couldn't get guild name, using guild ID instead");
        format!("'{guild_id}'")
    }
}

pub async fn guild_name_from_guild_id(ctx: Context<'_>, guild_id: GuildId) -> String {
    if let Some(value) = guild_id.name(ctx) {
        value
    } else {
        warn!("Couldn't get guild name, using guild ID instead");
        format!("'{guild_id}'")
    }
}

pub async fn guild_name(ctx: Context<'_>) -> String {
    guild(ctx).await.name
}

pub async fn guild_id_raw(ctx: &crate::serenity::Context) -> GuildId {
    super::current_application_info_raw(ctx)
        .await
        .expect("Couldn't get current application info")
        .guild_id
        .expect("Couldn't find guild ID in current application")
}

pub async fn guild_id(ctx: Context<'_>) -> GuildId {
    guild(ctx).await.id
}

pub fn guild_from_guild_id_raw(ctx: &crate::serenity::Context, guild_id: GuildId) -> Guild {
    if let Some(value) = ctx.cache.guild(guild_id) {
        value.clone()
    } else {
        panic!("Couldn't get guild from cache");
    }
}

pub async fn guild(ctx: Context<'_>) -> Guild {
    if let Some(value) = ctx.guild() {
        value.clone()
    } else {
        panic!("Couldn't get guild");
    }
}
