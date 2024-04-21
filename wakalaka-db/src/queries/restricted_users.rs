// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Timestamp, UserId};
use sqlx::{
    types::chrono::{DateTime, NaiveDateTime},
    Row, SqlitePool,
};
use tracing::error;
use wakalaka_core::types::SqlxThrowable;

pub async fn fetch_created_at_from_db(
    pool: &SqlitePool,
    user_id: &UserId,
) -> SqlxThrowable<NaiveDateTime> {
    let query = sqlx::query("SELECT created_at FROM restricted_users WHERE user_id = ?")
        .bind(i64::from(*user_id));

    let row = query.fetch_one(pool).await?;

    let created_at = row.get::<NaiveDateTime, _>("created_at");
    Ok(created_at)
}

pub async fn fetch_reason_from_db(pool: &SqlitePool, user_id: &UserId) -> SqlxThrowable<String> {
    let query = sqlx::query("SELECT reason FROM restricted_users WHERE user_id = ?")
        .bind(i64::from(*user_id));

    let row = query.fetch_one(pool).await?;

    let reason = row.get::<String, _>("reason");
    Ok(reason)
}

pub async fn fetch_user_id_from_db(pool: &SqlitePool, user_id: &UserId) -> SqlxThrowable<UserId> {
    let query = sqlx::query("SELECT user_id FROM restricted_users WHERE user_id = ?")
        .bind(i64::from(*user_id));

    let row = query.fetch_one(pool).await?;

    let user_id = UserId::from(row.get::<i64, _>("user_id") as u64);
    Ok(user_id)
}

pub async fn remove_restricted_user_from_db(
    pool: &SqlitePool,
    user_id: &UserId,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query("DELETE FROM restricted_users WHERE user_id = ?")
        .bind(i64::from(*user_id))
        .execute(pool);
    if let Err(e) = delete.await {
        error!("Failed to remove restricted user from database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn add_restricted_user_to_db(
    pool: &SqlitePool,
    user_id: &UserId,
    reason: impl Into<&String>,
    created_at: &Timestamp,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let insert =
        sqlx::query("INSERT INTO restricted_users (user_id, reason, created_at) VALUES (?, ?, ?)")
            .bind(i64::from(*user_id))
            .bind(reason.into())
            .bind(DateTime::from_timestamp(created_at.timestamp(), 0))
            .execute(pool);
    if let Err(e) = insert.await {
        let error = format!("{e:?}");
        if error.contains("1555") {
            // UNIQUE constraint failed
            return Ok(());
        }

        error!("Failed to add restricted user to database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
