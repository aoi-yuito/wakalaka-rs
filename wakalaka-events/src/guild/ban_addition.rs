// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, User};

use wakalaka_core::{
    accessors,
    types::{SerenityContext, Throwable},
};

pub(crate) async fn handle_guild_ban_addition_event(
    ctx: &SerenityContext,
    guild_id: &GuildId,
    user: &User,
) -> Throwable<()> {
    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    let user_name = &user.name;

    tracing::info!("@{user_name} banned from {guild_name}");

    Ok(())
}
