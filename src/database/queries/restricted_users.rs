
use serenity::all::UserId;
use sqlx::{Row, SqlitePool};

use crate::SqlxError;

pub(crate) async fn select_user_ids_from(
    db: &SqlitePool,
    user_id: &UserId,
) -> Result<Vec<UserId>, SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("SELECT user_id FROM restricted_users WHERE user_id = ?")
        .bind(i64::from(*user_id));

    let mut user_ids = vec![];

    let rows = query.fetch_all(db).await?;
    for row in rows {
        let user_id = UserId::from(row.get::<i64, _>("user_id") as u64);
        user_ids.push(user_id);
    }

    transaction.commit().await?;

    Ok(user_ids)
}

pub(crate) async fn select_user_id_from(
    db: &SqlitePool,
    user_id: &UserId,
) -> Result<Option<UserId>, SqlxError> {
    let query = sqlx::query("SELECT user_id FROM restricted_users WHERE user_id = ?")
        .bind(i64::from(*user_id));

    let row = query.fetch_one(db).await?;

    let user_id = UserId::from(row.get::<i64, _>("user_id") as u64);
    Ok(Some(user_id))
}

pub(crate) async fn delete_from(db: &SqlitePool, user_id: &UserId) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query =
        sqlx::query("DELETE FROM restricted_users WHERE user_id = ?").bind(i64::from(*user_id));
    query.execute(db).await?;

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn insert_into(
    db: &SqlitePool,
    user_id: &UserId,
    reason: &String,
) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("INSERT INTO restricted_users (user_id, reason) VALUES (?, ?)")
        .bind(i64::from(*user_id))
        .bind(reason);
    query.execute(db).await?;

    transaction.commit().await?;

    Ok(())
}
