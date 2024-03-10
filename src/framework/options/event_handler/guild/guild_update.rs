// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::PartialGuild;
use sqlx::SqlitePool;

use crate::{database::queries, Throwable};

pub(crate) async fn handle(db: &SqlitePool, guild: &PartialGuild) -> Throwable<()> {
    let guild_id = guild.id;

    let db_owner_id = queries::guilds::select_owner_id(db, &guild_id).await?;

    let guild_owner_id = guild.owner_id;
    if guild_owner_id != db_owner_id {
        queries::guilds::update_owner_id(db, &guild_id, &guild_owner_id).await?;
    }

    Ok(())
}
