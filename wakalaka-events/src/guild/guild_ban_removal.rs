// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, User};

use wakalaka_core::{
    accessors,
    types::{SContext, Throwable},
};

pub(crate) async fn handle_guild_ban_removal_event(
    ctx: &SContext,
    guild_id: &GuildId,
    user: &User,
) -> Throwable<()> {
    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    let user_name = &user.name;

    tracing::info!("@{user_name} unbanned from {guild_name}");

    Ok(())
}
