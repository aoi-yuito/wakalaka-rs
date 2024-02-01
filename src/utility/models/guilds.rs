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
    all::{Guild, GuildId, UserId},
    model::ModelError,
};
use tracing::{error, warn};

use crate::Context;

pub fn owner_id(ctx: Context<'_>) -> Result<UserId, ModelError> {
    Ok(guild(ctx)?.owner_id)
}

pub fn guild_name_from_guild_id_raw(ctx: &crate::serenity::Context, guild_id: GuildId) -> String {
    if let Some(guild_name) = guild_id.name(ctx) {
        guild_name
    } else {
        warn!("Couldn't get guild name, using guild ID instead");
        format!("'{guild_id}'")
    }
}

pub fn guild_name_from_guild_id(ctx: Context<'_>, guild_id: GuildId) -> String {
    if let Some(guild_name) = guild_id.name(ctx) {
        guild_name
    } else {
        warn!("Couldn't get guild name, using guild ID instead");
        format!("'{guild_id}'")
    }
}

pub async fn guild_name_raw(ctx: &crate::serenity::Context) -> Result<String, ModelError> {
    Ok(guild_raw(ctx).await?.name)
}

pub fn guild_name(ctx: Context<'_>) -> Result<String, ModelError> {
    Ok(guild(ctx)?.name)
}

pub async fn guild_id_raw(ctx: &crate::serenity::Context) -> GuildId {
    super::current_application_info_raw(ctx)
        .await
        .expect("Couldn't get current application info")
        .guild_id
        .expect("Couldn't find guild ID in current application")
}

pub fn guild_id(ctx: Context<'_>) -> Result<GuildId, ModelError> {
    Ok(guild(ctx)?.id)
}

pub async fn guild_raw(ctx: &crate::serenity::Context) -> Result<Guild, ModelError> {
    match guild_id_raw(ctx).await.to_guild_cached(ctx) {
        Some(guild) => Ok(guild.clone()),
        None => {
            error!("Couldn't get guild");
            return Err(ModelError::GuildNotFound);
        }
    }
}

pub fn guild(ctx: Context<'_>) -> Result<Guild, ModelError> {
    match ctx.guild() {
        Some(guild) => Ok(guild.clone()),
        None => {
            error!("Couldn't get guild");
            return Err(ModelError::GuildNotFound);
        }
    }
}
