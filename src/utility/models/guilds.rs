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

pub async fn owner_id_raw(
    ctx: &crate::serenity::Context,
    guild_id: GuildId,
) -> Result<UserId, ModelError> {
    match guild_id.to_partial_guild(ctx).await {
        Ok(guild) => Ok(guild.owner_id),
        Err(why) => {
            error!("Failed to get owner ID from guild ID: {why:?}");
            return Err(ModelError::GuildNotFound);
        }
    }
}

pub fn owner_id(ctx: Context<'_>) -> Result<UserId, ModelError> {
    Ok(guild(ctx)?.owner_id)
}

pub async fn guild_name_raw(ctx: &crate::serenity::Context, guild_id: GuildId) -> String {
    guild_id.name(ctx).map_or_else(
        || {
            warn!("Couldn't get guild name, using guild ID instead");
            format!("'{guild_id}'")
        },
        |guild_name| guild_name,
    )
}

pub fn guild_name(ctx: Context<'_>, guild_id: GuildId) -> String {
    guild_id.name(ctx).map_or_else(
        || {
            warn!("Couldn't get guild name, using guild ID instead");
            format!("'{guild_id}'")
        },
        |guild_name| guild_name,
    )
}

pub fn guild_id_from_component_raw(
    component: &ComponentInteraction,
) -> Result<GuildId, ModelError> {
    match component.guild_id {
        Some(guild_id) => Ok(guild_id),
        None => {
            error!("Failed to get guild ID from component");
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
            error!("Failed to get guild");
            return Err(ModelError::GuildNotFound);
        }
    }
}
