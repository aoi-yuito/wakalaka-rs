use serenity::all::UserId;
use sqlx::{Row, SqlitePool};
use tracing::{debug, error};

use crate::{SqlxError, SqlxThrowable};

pub(crate) async fn select_violations_from(
    db: &SqlitePool,
    user_id: &UserId,
) -> SqlxThrowable<i64> {
    let query =
        sqlx::query("SELECT violations FROM users WHERE user_id = ?").bind(i64::from(*user_id));

    let row = query.fetch_one(db).await?;

    let count = row.get::<i64, _>("violations");
    Ok(count)
}

pub(crate) async fn select_user_id_from(
    db: &SqlitePool,
    user_id: &UserId,
) -> SqlxThrowable<UserId> {
    let query =
        sqlx::query("SELECT user_id FROM users WHERE user_id = ?").bind(i64::from(*user_id));

    let row = query.fetch_one(db).await?;

    let user_id = UserId::from(row.get::<i64, _>("user_id") as u64);
    Ok(user_id)
}

pub(crate) async fn update_set_violations(
    db: &SqlitePool,
    user_id: &UserId,
    violations: i64,
) -> SqlxThrowable<()> {
    let transaction = db.begin().await?;

    let query = sqlx::query("UPDATE users SET violations = ? WHERE user_id = ?")
        .bind(violations)
        .bind(i64::from(*user_id));
    match query.execute(db).await {
        Ok(_) => {
            debug!("Updated Users:\n\tuser_id: {user_id}\n\tviolations: {violations}");
        }
        Err(why) => {
            transaction.rollback().await?;

            error!("Failed to update Users: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn insert_into(db: &SqlitePool, user_id: &UserId) -> SqlxThrowable<()> {
    let transaction = db.begin().await?;

    let query = sqlx::query("INSERT INTO users (user_id) VALUES (?)").bind(i64::from(*user_id));
    match query.execute(db).await {
        Ok(_) => {
            debug!("Inserted into Users:\n\tuser_id: {user_id}");
        }
        Err(why) => {
            let error = format!("{why}");
            if error.contains("1555") {
                // UNIQUE constraint failed
                return Ok(());
            }

            transaction.rollback().await?;

            error!("Failed to insert into Users: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

    transaction.commit().await?;

    Ok(())
}
