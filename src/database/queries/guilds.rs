use serenity::all::{GuildId, UserId};
use sqlx::{Row, SqlitePool};
use tracing::{debug, error};

use crate::{SqlxError, SqlxThrowable};

pub(crate) async fn select_owner_id_from(
    db: &SqlitePool,
    guild_id: &GuildId,
) -> SqlxThrowable<UserId> {
    let query =
        sqlx::query("SELECT owner_id FROM guilds WHERE guild_id = ?").bind(i64::from(*guild_id));

    let row = query.fetch_one(db).await?;

    let owner_id = UserId::from(row.get::<i64, _>("owner_id") as u64);
    Ok(owner_id)
}

pub(crate) async fn update_set_owner_id(
    db: &SqlitePool,
    guild_id: &GuildId,
    owner_id: &UserId,
) -> SqlxThrowable<()> {
    let transaction = db.begin().await?;

    let query = sqlx::query("UPDATE guilds SET owner_id = ? WHERE guild_id = ?")
        .bind(i64::from(*owner_id))
        .bind(i64::from(*guild_id));
    match query.execute(db).await {
        Ok(_) => {
            debug!("Updated Guilds:\n\tguild_id: {guild_id}\n\towner_id: {owner_id}");
        }
        Err(why) => {
            transaction.rollback().await?;

            error!("Failed to update Guilds: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn delete_from(db: &SqlitePool, guild_id: &GuildId) -> SqlxThrowable<()> {
    let transaction = db.begin().await?;

    let query = sqlx::query("DELETE FROM guilds WHERE guild_id = ?").bind(i64::from(*guild_id));
    match query.execute(db).await {
        Ok(_) => {
            debug!("Deleted from Guilds:\n\tguild_id: {guild_id}");
        }
        Err(why) => {
            transaction.rollback().await?;

            error!("Failed to delete from Guilds: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn insert_into(
    db: &SqlitePool,
    guild_id: &GuildId,
    owner_id: &UserId,
) -> SqlxThrowable<()> {
    let transaction = db.begin().await?;

    let query = sqlx::query("INSERT INTO guilds (guild_id, owner_id) VALUES (?, ?)")
        .bind(i64::from(*guild_id))
        .bind(i64::from(*owner_id));
    match query.execute(db).await {
        Ok(_) => {
            debug!("Inserted into Guilds:\n\tguild_id: {guild_id}\n\towner_id: {owner_id}");
        }
        Err(why) => {
            let error = format!("{why}");
            if error.contains("1555") {
                // UNIQUE constraint failed
                return Ok(());
            }

            transaction.rollback().await?;

            error!("Failed to insert into Guilds: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

    transaction.commit().await?;

    Ok(())
}
