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

use chrono::{NaiveDateTime, Utc};
use serenity::all::{GuildId, UserId};
use sqlx::{Row, SqlitePool};
use tokio::time::Instant;
use tracing::{error, info};

pub(crate) enum InfractionType {
    Warn,
    Deaf,
    Mute,
    Ban,
}

impl InfractionType {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Warn => "warning",
            Self::Deaf => "deafen",
            Self::Mute => "mute",
            Self::Ban => "ban",
        }
    }
}

pub(crate) async fn warnings(
    user_id: UserId,
    guild_id: GuildId,
    infraction_type: &'static str,
    pool: &SqlitePool,
) -> Result<Vec<(i32, String, i64, String, NaiveDateTime, NaiveDateTime, bool)>, sqlx::Error> {
    let start_time = Instant::now();

    let infract_query = sqlx::query(
        "SELECT id, type, moderator_id, reason, created_at, expires_at, active FROM infractions WHERE user_id = ? AND guild_id = ? AND type = ?",
    )
    .bind(i64::from(user_id))
    .bind(i64::from(guild_id))
    .bind(infraction_type);

    let mut infracts = Vec::new();

    let rows = infract_query.fetch_all(pool).await?;
    for row in rows {
        if row.is_empty() {
            return Ok(infracts);
        }

        let id = row.get::<i32, _>(0);
        let infraction_type = row.get::<String, _>(1);
        let moderator_id = row.get::<i64, _>(2);
        let reason = row.get::<String, _>(3);
        let created_at = row.get::<NaiveDateTime, _>(4);
        let expires_at = row.get::<NaiveDateTime, _>(5);
        let active = row.get::<bool, _>(6);

        infracts.push((
            id,
            infraction_type,
            moderator_id,
            reason,
            created_at,
            expires_at,
            active,
        ));
    }

    let elapsed_time = start_time.elapsed();
    info!("Got warnings from database in {elapsed_time:.2?}");

    Ok(infracts)
}

pub(crate) async fn update_infraction(
    user_id: UserId,
    moderator_id: UserId,
    guild_id: GuildId,
    reason: &String,
    created_at: Option<NaiveDateTime>,
    expires_at: Option<NaiveDateTime>,
    active: bool,
    pool: &SqlitePool,
) {
    let start_time = Instant::now();

    let created_at = created_at.unwrap_or_else(|| Utc::now().naive_utc());
    let expires_at = expires_at.unwrap_or_else(|| Utc::now().naive_utc());

    let infract_query = sqlx::query(
        "UPDATE infractions SET moderator_id = ?, guild_id = ?, reason = ?, created_at = ?, expires_at = ?, active = ? WHERE user_id = ?",
    )
    .bind(i64::from(moderator_id))
    .bind(i64::from(guild_id))
    .bind(reason)
    .bind(created_at)
    .bind(expires_at)
    .bind(active)
    .bind(i64::from(user_id));

    if let Err(why) = infract_query.execute(pool).await {
        error!("Couldn't update infraction in database: {why:?}");
        return;
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Updated infraction in database in {elapsed_time:.2?}");
    }
}

pub(crate) async fn delete_infraction(id: i32, infraction_type: &'static str, pool: &SqlitePool) {
    let start_time = Instant::now();

    let infract_query = sqlx::query("DELETE FROM infractions WHERE id = ? AND type = ?")
        .bind(id)
        .bind(infraction_type);

    if let Err(why) = infract_query.execute(pool).await {
        error!("Couldn't delete infraction from database: {why:?}");
        return;
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Deleted infraction from database in {elapsed_time:.2?}");
    }
}

pub(crate) async fn insert_infraction(
    user_id: UserId,
    infraction_type: &'static str,
    moderator_id: UserId,
    guild_id: GuildId,
    reason: &String,
    created_at: Option<NaiveDateTime>,
    expires_at: Option<NaiveDateTime>,
    active: bool,
    pool: &SqlitePool,
) {
    let start_time = Instant::now();

    let created_at = created_at.unwrap_or(Utc::now().naive_utc());
    let expires_at = expires_at.unwrap_or(Utc::now().naive_utc());

    let infract_query = sqlx::query(
        "INSERT INTO infractions (user_id, type, moderator_id, guild_id, reason, created_at, expires_at, active) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(i64::from(user_id))
    .bind(infraction_type)
    .bind(i64::from(moderator_id))
    .bind(i64::from(guild_id))
    .bind(reason)
    .bind(created_at)
    .bind(expires_at)
    .bind(active);

    if let Err(why) = infract_query.execute(pool).await {
        error!("Couldn't insert infraction into database: {why:?}");
        return;
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Inserted infraction into database in {elapsed_time:.2?}");
    }
}
