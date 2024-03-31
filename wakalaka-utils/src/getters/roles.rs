// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, ModelError, Role, RoleId};
use wakalaka_core::types::{Context, Throwable};

use super::guilds;

pub async fn role_raw(ctx: Context<'_>, guild_id: &GuildId, role_id: &RoleId) -> Throwable<Role> {
    let guild = guilds::guild_cached(ctx, guild_id)?;

    let role = guild
        .roles
        .get(&role_id)
        .ok_or_else(|| Box::new(ModelError::RoleNotFound))?;
    Ok(role.clone())
}
