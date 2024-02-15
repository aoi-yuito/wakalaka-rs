
use serenity::all::UserId;
use sqlx::{Row, SqlitePool};

use crate::SqlxError;

pub(crate) async fn select_user_id_from(
    db: &SqlitePool,
    user_id: &UserId,
) -> Result<UserId, SqlxError> {
    let query =
        sqlx::query("SELECT user_id FROM users WHERE user_id = ?").bind(i64::from(*user_id));

    let row = query.fetch_one(db).await?;

    let user_id = UserId::from(row.get::<i64, _>("user_id") as u64);
    Ok(user_id)
}

pub(crate) async fn update_set_user_id(db: &SqlitePool, user_id: &UserId) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("UPDATE users SET user_id = ? WHERE user_id = ?")
        .bind(i64::from(*user_id))
        .bind(i64::from(*user_id));
    query.execute(db).await?;

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn delete_from(db: &SqlitePool, user_id: &UserId) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("DELETE FROM users WHERE user_id = ?").bind(i64::from(*user_id));
    query.execute(db).await?;

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn insert_into(db: &SqlitePool, user_id: &UserId) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("INSERT INTO users (user_id) VALUES (?)").bind(i64::from(*user_id));
    query.execute(db).await?;

    transaction.commit().await?;

    Ok(())
}
