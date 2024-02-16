// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Guild, UnavailableGuild, UserId};
use sqlx::SqlitePool;
use tracing::warn;

use crate::{database::queries, utils::models, Error, SContext};

pub(crate) async fn handle(
    ctx: &SContext,
    db: &SqlitePool,
    unavailable_guild: &UnavailableGuild,
    guild: &Option<Guild>,
) -> Result<(), Error> {
    if unavailable_guild.unavailable {
        let unavailable_guild_id = unavailable_guild.id;
        let unavailable_guild_name = models::guilds::name_raw(&ctx, &unavailable_guild_id);

        warn!("{unavailable_guild_name} isn't available, skipping ...");
        return Ok(());
    }

    let guild = guild.as_ref().unwrap();
    let guild_id = guild.id;
    let guild_name = &guild.name;

    let guild_owner_id = guild.owner_id;

    let deleted_user_id = UserId::from(456226577798135808);

    if guild_owner_id == deleted_user_id {
        warn!("Owner of {guild_name} doesn't exist, removing entries ...");

        queries::guilds::delete_from(db, &guild_id).await?;
        queries::restricted_guilds::delete_from(db, &guild_id).await?;

        return Ok(());
    }

    queries::guilds::delete_from(db, &guild_id).await?;

    Ok(())
}
