// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Guild, UnavailableGuild, UserId};
use sqlx::PgPool;
use tracing::{info, warn};
use wakalaka_core::types::{SContext, Throwable};
use wakalaka_utils::accessors;

use wakalaka_db::queries;

pub(crate) async fn handle_guild_delete_event(
    ctx: &SContext,
    unavailable_guild: &UnavailableGuild,
    guild: &Option<Guild>,
    pool: &PgPool,
) -> Throwable<()> {
    if unavailable_guild.unavailable {
        let unavailable_guild_id = &unavailable_guild.id;
        let unavailable_guild_name =
            accessors::guilds::fetch_raw_guild_name(ctx, unavailable_guild_id);

        warn!("{unavailable_guild_name} is not available, skipping ...");

        return Ok(());
    }

    let bot = accessors::users::fetch_raw_bot_user_info(ctx).await?;
    let bot_name = &bot.name;

    let guild = guild.as_ref().expect("Guild is not available");
    let guild_id = &guild.id;
    let guild_name = &guild.name;

    let guild_owner_id = &guild.owner_id;

    let deleted_user_id = UserId::from(456226577798135808);

    if guild_owner_id == &deleted_user_id {
        warn!("{guild_name} is missing owner, removing ...");

        queries::guilds::remove_guild_from_db(pool, guild_id).await?;

        queries::restricted_guilds::remove_restricted_guild_from_db(pool, guild_id).await?;

        return Ok(());
    }

    info!("@{bot_name} left {guild_name}");

    queries::guilds::remove_guild_from_db(pool, guild_id).await?;

    Ok(())
}
