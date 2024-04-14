// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, RoleId};
use tracing::info;
use wakalaka_core::types::{SContext, Throwable};
use wakalaka_utils::accessors;

pub(crate) async fn handle_guild_role_delete_event(
    ctx: &SContext,
    guild_id: &GuildId,
    role_id: &RoleId,
) -> Throwable<()> {
    let role = accessors::roles::fetch_raw_cached_role(ctx, guild_id, role_id).await?;
    let role_name = &role.name;

    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    info!("@{role_name:?} deleted in {guild_name:?}");

    Ok(())
}
