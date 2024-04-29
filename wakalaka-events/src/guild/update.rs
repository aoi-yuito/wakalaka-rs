// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::PartialGuild;
use sqlx::PgPool;

use wakalaka_core::types::Throwable;
use wakalaka_database::queries;

pub(crate) async fn handle_guild_update_event(
    partial_guild: &PartialGuild,
    pool: &PgPool,
) -> Throwable<()> {
    let guild_id = &partial_guild.id;

    let guild_owner_id = &partial_guild.owner_id;

    let db_owner_id = queries::guilds::fetch_owner_id_from_db(pool, guild_id).await?;

    if *guild_owner_id != db_owner_id {
        queries::guilds::update_owner_id_in_db(pool, guild_id, guild_owner_id).await?;
    }

    Ok(())
}
