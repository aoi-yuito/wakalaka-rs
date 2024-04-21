// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, User};
use sqlx::SqlitePool;
use tracing::info;
use wakalaka_core::types::{SContext, Throwable};
use wakalaka_utils::accessors;

use wakalaka_db::queries;

pub(crate) async fn handle_guild_member_removal_event(
    ctx: &SContext,
    guild_id: &GuildId,
    user: &User,
    pool: &SqlitePool,
) -> Throwable<()> {
    if user.bot || user.system {
        return Ok(());
    }

    let user_id = &user.id;
    let user_name = &user.name;

    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    info!("@{user_name} left {guild_name}");

    queries::users::remove_user_from_db(pool, user_id).await?;

    Ok(())
}
