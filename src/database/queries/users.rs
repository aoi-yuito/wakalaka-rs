// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::UserId;
use sqlx::{Row, SqlitePool};
use tracing::error;
use wakalaka_core::types::SqlxThrowable;

pub async fn fetch_infractions_from_db(pool: &SqlitePool, user_id: &UserId) -> SqlxThrowable<i64> {
    let query =
        sqlx::query("SELECT infractions FROM users WHERE user_id = ?").bind(i64::from(*user_id));

    let row = query.fetch_one(pool).await?;

    let infractions = row.get::<i64, _>("infractions");

    Ok(infractions)
}

pub async fn fetch_user_id_from_db(pool: &SqlitePool, user_id: &UserId) -> SqlxThrowable<UserId> {
    let query =
        sqlx::query("SELECT user_id FROM users WHERE user_id = ?").bind(i64::from(*user_id));

    let row = query.fetch_one(pool).await?;

    let user_id = UserId::from(row.get::<i64, _>("user_id") as u64);

    Ok(user_id)
}

pub async fn remove_user_from_db(pool: &SqlitePool, user_id: &UserId) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query("DELETE FROM users WHERE user_id = ?")
        .bind(i64::from(*user_id))
        .execute(pool);
    if let Err(e) = delete.await {
        error!("Failed to delete from users: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn add_user_to_db(pool: &SqlitePool, user_id: &UserId) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let insert = sqlx::query("INSERT INTO users (user_id) VALUES (?)")
        .bind(i64::from(*user_id))
        .execute(pool);
    if let Err(e) = insert.await {
        error!("Failed to insert into users: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
