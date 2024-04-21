// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, User};
use tracing::info;
use wakalaka_core::types::{SContext, Throwable};
use wakalaka_utils::accessors;

pub(crate) async fn handle_guild_ban_addition_event(
    ctx: &SContext,
    guild_id: &GuildId,
    user: &User,
) -> Throwable<()> {
    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    let user_name = &user.name;

    info!("@{user_name} banned from {guild_name}");

    Ok(())
}
