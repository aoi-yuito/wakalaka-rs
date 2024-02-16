use serenity::all::UserId;
use sqlx::{Row, SqlitePool};
use tracing::error;

use crate::SqlxError;

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
    match query.execute(db).await {
        Ok(_) => (),
        Err(why) => {
            let error = format!("{why}");
            if error.contains("1555") {
                // UNIQUE constraint failed
                return Ok(());
            }
            
            transaction.rollback().await?;

            error!("Failed to delete from RestrictedUsers: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

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
        .bind(reason.trim());
    match query.execute(db).await {
        Ok(_) => (),
        Err(why) => {
            transaction.rollback().await?;

            error!("Failed to insert into RestrictedUsers: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

    transaction.commit().await?;

    Ok(())
}
