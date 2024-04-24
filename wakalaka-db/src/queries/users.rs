// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Timestamp, UserId};
use sqlx::PgPool;
use wakalaka_utils::converters::times;

use wakalaka_core::types::SqlxThrowable;

pub async fn update_warnings_in_db(
    pool: &PgPool,
    user_id: &UserId,
    warnings: i32,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let update = sqlx::query!(
        "UPDATE users SET warnings = $1 WHERE user_id = $2",
        warnings,
        i64::from(*user_id)
    )
    .execute(pool);
    if let Err(e) = update.await {
        tracing::error!("Failed to UPDATE users: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn fetch_warnings_from_db(pool: &PgPool, user_id: &UserId) -> SqlxThrowable<i32> {
    let select = sqlx::query!(
        "SELECT warnings FROM users WHERE user_id = $1",
        i64::from(*user_id)
    );

    let row = select.fetch_one(pool).await?;

    let warnings = row.warnings;
    Ok(warnings)
}

pub async fn fetch_user_id_from_db(pool: &PgPool, user_id: &UserId) -> SqlxThrowable<UserId> {
    let select = sqlx::query!(
        "SELECT user_id FROM users WHERE user_id = $1",
        i64::from(*user_id)
    );

    let row = select.fetch_one(pool).await?;

    let user_id = UserId::from(row.user_id as u64);
    Ok(user_id)
}

pub async fn remove_user_from_db(pool: &PgPool, user_id: &UserId) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete =
        sqlx::query!("DELETE FROM users WHERE user_id = $1", i64::from(*user_id)).execute(pool);
    if let Err(e) = delete.await {
        tracing::error!("Failed to DELETE FROM users: {e:?}");

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

    let insert = sqlx::query!(
        "INSERT INTO users (user_id, created_at) VALUES ($1, $2) ON CONFLICT (user_id) DO NOTHING",
        i64::from(*user_id),
        times::datetime_to_naivedatetime(created_at)
    )
    .execute(pool);
    if let Err(e) = insert.await {
        tracing::error!("Failed to INSERT INTO users: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
