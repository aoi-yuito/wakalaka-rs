// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

use serenity::all::GuildId;
use sqlx::{Row, SqlitePool};
use tokio::time::Instant;
use tracing::{debug, error};

#[macro_export]
macro_rules! check_restricted_guild {
    (&$pool:expr, &$guild_id:expr) => {{
        match crate::database::restricted_guilds::select_guild_id_from_restricted_guilds(
            &$guild_id, &$pool,
        )
        .await
        {
            Ok(true) => true,
            Ok(false) => false,
            Err(_) => false,
        }
    }};
}

pub async fn select_guild_id_from_restricted_guilds(
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Result<bool, sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "SELECT guild_id FROM restricted_guilds
        WHERE guild_id = ?",
    )
    .bind(i64::from(*guild_id));
    let row = match query.fetch_one(pool).await {
        Ok(row) => row,
        Err(why) => {
            if why.to_string().contains("no such table") {
                return Ok(false);
            }

            return Err(why);
        }
    };

    let elapsed_time = start_time.elapsed();
    debug!("Selected from RestrictedGuilds in {elapsed_time:.2?}",);

    Ok(row.get::<i64, _>(0) as u64 == u64::from(*guild_id))
}

pub async fn delete_from_restricted_guilds(
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "DELETE FROM restricted_guilds
        WHERE guild_id = ?",
    )
    .bind(i64::from(*guild_id));
    if let Err(why) = query.execute(pool).await {
        error!("Failed to delete from RestrictedGuilds: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Deleted from RestrictedGuilds in {elapsed_time:.2?}");

    Ok(())
}

pub async fn insert_into_restricted_guilds(
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "INSERT INTO restricted_guilds (
            guild_id
        ) VALUES (?)",
    )
    .bind(i64::from(*guild_id));
    if let Err(why) = query.execute(pool).await {
        if why.to_string().contains("1555") {
            // UNIQUE constraint failed
            return Ok(());
        }

        error!("Failed to insert into RestrictedGuilds: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Inserted into RestrictedGuilds in {elapsed_time:.2?}");

    Ok(())
}
