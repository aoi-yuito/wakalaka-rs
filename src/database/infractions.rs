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
use serenity::all::UserId;
use sqlx::SqlitePool;
use tokio::time::Instant;
use tracing::{error, info};

pub(crate) async fn update_infraction(
    user_id: UserId,
    moderator_id: UserId,
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
        "UPDATE infractions SET moderator_id = ?, reason = ?, created_at = ?, expires_at = ?, active = ? WHERE user_id = ?",
    )
    .bind(i64::from(moderator_id))
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

pub(crate) async fn insert_infractions(
    user_id: UserId,
    infraction_type: &'static str,
    moderator_id: UserId,
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
        "INSERT INTO infractions (user_id, type, moderator_id, reason, created_at, expires_at, active) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(i64::from(user_id))
    .bind(infraction_type)
    .bind(i64::from(moderator_id))
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
