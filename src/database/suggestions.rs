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

pub(crate) async fn delete_suggest(message_id: i64, guild_id: i64, pool: &SqlitePool) {
    let start_time = Instant::now();

    let suggest_query =
        sqlx::query("DELETE FROM suggestions WHERE message_id = ? AND guild_id = ?")
            .bind(message_id)
            .bind(guild_id);

    if let Err(why) = suggest_query.execute(pool).await {
        error!("Couldn't delete suggestion from database: {why:?}");
        return;
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Deleted suggestion from database in {elapsed_time:.2?}");
    }
}

pub(crate) async fn update_suggest(
    message_id: i64,
    guild_id: i64,
    user_id: i64,
    moderator_id: i64,
    created_at: NaiveDateTime,
    accepted_at: Option<NaiveDateTime>,
    rejected_at: Option<NaiveDateTime>,
    pool: &SqlitePool,
) {
    let start_time = Instant::now();

    let suggest_query = sqlx::query(
        "UPDATE suggestions SET user_id = ?, moderator_id = ?, created_at = ?, accepted_at = ?, rejected_at = ? WHERE message_id = ? AND guild_id = ?",
    ).bind(user_id).bind(moderator_id).bind(created_at).bind(accepted_at).bind(rejected_at).bind(message_id).bind(guild_id);

    if let Err(why) = suggest_query.execute(pool).await {
        error!("Couldn't update suggestion within database: {why:?}");
        return;
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Updated suggestion within database in {elapsed_time:.2?}");
    }
}

pub(crate) async fn insert_suggest(
    message_id: i64,
    guild_id: i64,
    user_id: i64,
    moderator_id: i64,
    created_at: NaiveDateTime,
    accepted_at: Option<NaiveDateTime>,
    rejected_at: Option<NaiveDateTime>,
    pool: &SqlitePool,
) {
    let start_time = Instant::now();

    let suggest_query = sqlx::query(
        "INSERT INTO suggestions (message_id, guild_id, user_id, moderator_id, created_at, accepted_at, rejected_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    ).bind(message_id).bind(guild_id).bind(user_id).bind(moderator_id).bind(created_at).bind(accepted_at).bind(rejected_at);

    if let Err(why) = suggest_query.execute(pool).await {
        error!("Couldn't insert suggestion into database: {why:?}");
        return;
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Inserted suggestion into database in {elapsed_time:.2?}");
    }
}
