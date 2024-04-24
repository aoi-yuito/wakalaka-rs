// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Timestamp, UserId};
use sqlx::{types::chrono::NaiveDateTime, PgPool};

use wakalaka_core::types::SqlxThrowable;
use wakalaka_utils::converters::times;

pub async fn fetch_created_at_from_db(
    pool: &PgPool,
    user_id: &UserId,
) -> SqlxThrowable<NaiveDateTime> {
    let select = sqlx::query!(
        "SELECT created_at FROM restricted_users WHERE user_id = $1",
        i64::from(*user_id)
    );

    let row = select.fetch_one(pool).await?;

    let created_at = row.created_at;
    Ok(created_at)
}

pub async fn fetch_reason_from_db(pool: &PgPool, user_id: &UserId) -> SqlxThrowable<String> {
    let select = sqlx::query!(
        "SELECT reason FROM restricted_users WHERE user_id = $1",
        i64::from(*user_id)
    );

    let row = select.fetch_one(pool).await?;

    let reason = row.reason;
    Ok(reason)
}

pub async fn fetch_user_id_from_db(pool: &PgPool, user_id: &UserId) -> SqlxThrowable<UserId> {
    let select = sqlx::query!(
        "SELECT user_id FROM restricted_users WHERE user_id = $1",
        i64::from(*user_id)
    );

    let row = select.fetch_one(pool).await?;

    let user_id = UserId::from(row.user_id as u64);
    Ok(user_id)
}

pub async fn remove_restricted_user_from_db(pool: &PgPool, user_id: &UserId) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query!(
        "DELETE FROM restricted_users WHERE user_id = $1",
        i64::from(*user_id)
    )
    .execute(pool);
    if let Err(e) = delete.await {
        tracing::error!("Failed to DELETE FROM restricted_users: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn add_restricted_user_to_db(
    pool: &PgPool,
    user_id: &UserId,
    reason: impl Into<&String>,
    created_at: &Timestamp,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let insert = sqlx::query!(
        "INSERT INTO restricted_users (user_id, reason, created_at) VALUES ($1, $2, $3) ON CONFLICT (user_id) DO NOTHING",
        i64::from(*user_id),
        reason.into(),
        times::datetime_to_naivedatetime(created_at)
    ).execute(pool);
    if let Err(e) = insert.await {
        tracing::error!("Failed to INSERT INTO restricted_users: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
