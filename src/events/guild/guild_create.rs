// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Guild;
use sqlx::PgPool;
use tracing::info;
use wakalaka_core::types::{SContext, Throwable};
use wakalaka_utils::accessors;

use wakalaka_db::{self, checks, queries};

pub(crate) async fn handle_guild_create_event(
    ctx: &SContext,
    guild: &Guild,
    is_new: &Option<bool>,
    pool: &PgPool,
) -> Throwable<()> {
    if !is_new.is_some() {
        return Ok(());
    }

    let guild_restricted = checks::is_guild_restricted(ctx, pool, guild).await?;
    if guild_restricted {
        guild.leave(ctx).await?;

        return Ok(());
    }

    let bot = accessors::users::fetch_raw_bot_user_info(ctx).await?;
    let bot_name = &bot.name;

    let guild_id = &guild.id;
    let guild_name = &guild.name;

    let guild_owner_id = &guild.owner_id;
    let guild_owner_created_at = &guild_owner_id.created_at();

    let guild_created_at = &guild_id.created_at();

    info!("@{bot_name} joined {guild_name}");

    queries::users::add_user_to_db(pool, guild_owner_id, guild_owner_created_at).await?;

    queries::guilds::add_guild_to_db(pool, guild_id, guild_owner_id, guild_created_at).await?;

    Ok(())
}
