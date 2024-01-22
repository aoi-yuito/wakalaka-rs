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

use chrono::NaiveDateTime;
use serenity::all::{GuildId, UserId};
use sqlx::{Row, SqlitePool};
use tokio::time::Instant;
use tracing::{error, info};
use uuid::Uuid;

pub(crate) enum InfractionType {
    Warn,
    Deaf,
    Mute,
    Timeout,
    Ban,
}

impl InfractionType {
    fn as_str(&self) -> &str {
        match self {
            Self::Warn => "warning",
            Self::Deaf => "deafen",
            Self::Mute => "mute",
            Self::Timeout => "timeout",
            Self::Ban => "ban",
        }
    }
}

pub(crate) async fn select_from_infractions(
    infraction: InfractionType,
    user_id: &UserId,
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Result<Vec<(String, String, i64, i64, String, String, i64)>, sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "SELECT uuid, type, user_id, moderator_id, reason, created_at, guild_id FROM infractions WHERE type = ? AND user_id = ? AND guild_id = ?",
    )
    .bind(infraction.as_str())
    .bind(i64::from(*user_id))
    .bind(i64::from(*guild_id));

    let mut infractions = Vec::new();

    let rows = query.fetch_all(pool).await?;
    for row in rows {
        if row.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }

        let (uuid, infraction_type, user_id, moderator_id, reason, created_at, guild_id) = (
            row.get::<String, _>(0),
            row.get::<String, _>(1),
            row.get::<i64, _>(2),
            row.get::<i64, _>(3),
            row.get::<String, _>(4),
            row.get::<NaiveDateTime, _>(5),
            row.get::<i64, _>(6),
        );

        infractions.push((
            uuid,
            infraction_type,
            user_id,
            moderator_id,
            reason,
            created_at.to_string(),
            guild_id,
        ));
    }

    let elapsed_time = start_time.elapsed();
    info!("Selected from Infractions in {elapsed_time:.2?}");

    Ok(infractions)
}

pub(crate) async fn delete_from_infractions(
    uuid: &String,
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query("DELETE FROM infractions WHERE uuid = ? AND guild_id = ?")
        .bind(uuid)
        .bind(i64::from(*guild_id));
    if let Err(why) = query.execute(pool).await {
        error!("Couldn't delete from Infractions: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    info!("Deleted from Infractions in {elapsed_time:.2?}");

    Ok(())
}

pub(crate) async fn insert_into_infractions(
    infraction: InfractionType,
    user_id: &UserId,
    moderator_id: &UserId,
    reason: &String,
    created_at: NaiveDateTime,
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let uuid = Uuid::new_v4().to_string();

    let query = sqlx::query(
        "INSERT INTO infractions (uuid, type, user_id, moderator_id, reason, created_at, guild_id) VALUES (?, ?, ?, ?, ?, ?, ?)",
    ).bind(uuid)
    .bind(infraction.as_str())
    .bind(i64::from(*user_id))
    .bind(i64::from(*moderator_id))
    .bind(reason)
    .bind(created_at)
    .bind(i64::from(*guild_id));
    if let Err(why) = query.execute(pool).await {
        error!("Couldn't insert into Infractions: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    info!("Inserted into Infractions in {elapsed_time:.2?}");

    Ok(())
}
