// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, ModelError, Role, RoleId};
use tracing::warn;

use crate::{SContext, Throwable};

use super::guilds;

pub(crate) async fn role_from_id_raw(
    ctx: &SContext,
    guild_id: &GuildId,
    role_id: &RoleId,
) -> Throwable<Role> {
    let guild = guilds::guild_from_id_raw(ctx, guild_id)?;
    let guild_name = &guild.name;

    let role = match guild.roles.get(role_id) {
        Some(role) => role.clone(),
        None => {
            warn!("No role found in {guild_name}");
            return Err(Box::new(ModelError::RoleNotFound));
        }
    };
    Ok(role)
}
