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
    all::{ComponentInteraction, Guild, GuildId, UserId},
    model::ModelError,
};
use tracing::{error, warn};

use crate::Context;

pub async fn owner_id_from_guild_id_raw(
    ctx: &crate::serenity::Context,
    guild_id: GuildId,
) -> Result<UserId, ModelError> {
    match ctx.http.get_guild(guild_id).await {
        Ok(guild) => Ok(guild.owner_id),
        Err(why) => {
            error!("Couldn't get guild owner ID from guild ID: {why:?}");
            return Err(ModelError::MemberNotFound);
        }
    }
}

pub fn owner_id(ctx: Context<'_>) -> Result<UserId, ModelError> {
    Ok(guild(ctx)?.owner_id)
}

pub async fn guild_name_from_guild_id_raw(
    ctx: &crate::serenity::Context,
    guild_id: GuildId,
) -> Option<String> {
    match ctx.http.get_guild(guild_id).await {
        Ok(guild) => Some(guild.name),
        Err(why) => {
            error!("Couldn't get guild name from guild ID: {why:?}");
            Some(format!("'{guild_id}'"))
        }
    }
}

pub fn guild_name_from_guild_id(ctx: Context<'_>, guild_id: GuildId) -> String {
    if let Some(guild_name) = guild_id.name(ctx) {
        guild_name
    } else {
        warn!("Couldn't get guild name from guild ID, using guild ID instead");
        format!("'{guild_id}'")
    }
}

pub fn guild_name(ctx: Context<'_>) -> Result<String, ModelError> {
    Ok(guild(ctx)?.name)
}

pub fn guild_id_from_component_raw(
    component: &ComponentInteraction,
) -> Result<GuildId, ModelError> {
    match component.guild_id {
        Some(guild_id) => Ok(guild_id),
        None => {
            warn!("Couldn't get guild ID from component");
            return Err(ModelError::GuildNotFound);
        }
    }
}

pub fn guild_id(ctx: Context<'_>) -> Result<GuildId, ModelError> {
    Ok(guild(ctx)?.id)
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
