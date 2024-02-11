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
use sqlx::SqlitePool;
use tokio::time::Instant;
use tracing::{error, info};

pub async fn delete_from_suggestions(message_id: i64, guild_id: i64, pool: &SqlitePool) {
    let start_time = Instant::now();

    let suggest_query =
        sqlx::query("DELETE FROM suggestions WHERE message_id = ? AND guild_id = ?")
            .bind(message_id)
            .bind(guild_id);

    if let Err(why) = suggest_query.execute(pool).await {
        error!("Failed to delete suggestion from database: {why:?}");
        return;
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Deleted suggestion from database in {elapsed_time:.2?}");
    }
}

pub async fn update_suggestions(
    moderator_id: i64,
    message_id: i64,
    guild_id: i64,
    accepted_at: Option<NaiveDateTime>,
    rejected_at: Option<NaiveDateTime>,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "UPDATE suggestions SET moderator_id = ?, accepted_at = ?, rejected_at = ? WHERE message_id = ? AND guild_id = ?",
    )
    .bind(moderator_id)
    .bind(accepted_at)
    .bind(rejected_at)
    .bind(message_id)
    .bind(guild_id);
    if let Err(why) = query.execute(pool).await {
        error!("Failed to update suggestion in database: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    info!("Updated suggestion in database in {elapsed_time:.2?}");

    Ok(())
}

pub async fn insert_into_suggestions(
    uuid: &String,
    user_id: i64,
    moderator_id: i64,
    created_at: NaiveDateTime,
    accepted_at: Option<NaiveDateTime>,
    rejected_at: Option<NaiveDateTime>,
    message_id: i64,
    channel_id: i64,
    guild_id: i64,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "INSERT INTO suggestions (uuid, user_id, moderator_id, created_at, accepted_at, rejected_at, message_id, channel_id, guild_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(uuid)
    .bind(user_id)
    .bind(moderator_id)
    .bind(created_at)
    .bind(accepted_at)
    .bind(rejected_at)
    .bind(message_id)
    .bind(channel_id)
    .bind(guild_id);
    if let Err(why) = query.execute(pool).await {
        error!("Failed to insert into Suggestions: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    info!("Inserted into Suggestions in {elapsed_time:.2?}");

    Ok(())
}
