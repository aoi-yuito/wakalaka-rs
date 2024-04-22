// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Timestamp, UserId};
use sqlx::{types::chrono::DateTime, PgPool, Row};
use tracing::error;
use wakalaka_core::types::SqlxThrowable;

pub async fn fetch_infractions_from_db(pool: &PgPool, user_id: &UserId) -> SqlxThrowable<i64> {
    let query =
        sqlx::query("SELECT infractions FROM users WHERE user_id = $1").bind(i64::from(*user_id));

    let row = query.fetch_one(pool).await?;

    let infractions = row.get::<i64, _>("infractions");
    Ok(infractions)
}

pub async fn fetch_user_id_from_db(pool: &PgPool, user_id: &UserId) -> SqlxThrowable<UserId> {
    let query =
        sqlx::query("SELECT user_id FROM users WHERE user_id = $1").bind(i64::from(*user_id));

    let row = query.fetch_one(pool).await?;

    let user_id = UserId::from(row.get::<i64, _>("user_id") as u64);
    Ok(user_id)
}

pub async fn remove_user_from_db(pool: &PgPool, user_id: &UserId) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query("DELETE FROM users WHERE user_id = $1")
        .bind(i64::from(*user_id))
        .execute(pool);
    if let Err(e) = delete.await {
        error!("Failed to remove user from database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn add_user_to_db(
    pool: &PgPool,
    user_id: &UserId,
    created_at: &Timestamp,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let insert = sqlx::query(
        "INSERT INTO users (user_id, created_at) VALUES ($1, $2) ON CONFLICT (user_id) DO NOTHING",
    )
    .bind(i64::from(*user_id))
    .bind(DateTime::from_timestamp(created_at.timestamp(), 0))
    .execute(pool);
    if let Err(e) = insert.await {
        error!("Failed to add user to database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
