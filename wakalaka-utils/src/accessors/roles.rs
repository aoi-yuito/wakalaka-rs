// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, ModelError, Role, RoleId};
use wakalaka_core::types::{Context, SContext, Throwable};

use super::guilds;

pub async fn fetch_role(
    ctx: Context<'_>,
    guild_id: &GuildId,
    role_id: &RoleId,
) -> Throwable<Role> {
    let guild = guilds::fetch_cached_guild(ctx, guild_id)?;

    let role = guild
        .roles
        .get(&role_id)
        .ok_or_else(|| Box::new(ModelError::RoleNotFound))?;
    Ok(role.clone())
}

pub async fn fetch_raw_role(
    ctx: &SContext,
    guild_id: &GuildId,
    role_id: &RoleId,
) -> Throwable<Role> {
    let guild = guilds::fetch_raw_cached_guild(ctx, guild_id)?;

    let role = guild
        .roles
        .get(&role_id)
        .ok_or_else(|| Box::new(ModelError::RoleNotFound))?;
    Ok(role.clone())
}
