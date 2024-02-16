use serenity::all::GuildId;
use sqlx::{Row, SqlitePool};
use tracing::info;

use crate::SqlxError;

pub(crate) async fn select_guild_id_from(
    db: &SqlitePool,
    guild_id: &GuildId,
) -> Result<GuildId, SqlxError> {
    let query = sqlx::query("SELECT guild_id FROM restricted_guilds WHERE guild_id = ?")
        .bind(i64::from(*guild_id));

    let row = query.fetch_one(db).await?;

    let guild_id = GuildId::from(row.get::<i64, _>("guild_id") as u64);
    Ok(guild_id)
}

pub(crate) async fn delete_from(db: &SqlitePool, guild_id: &GuildId) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query =
        sqlx::query("DELETE FROM restricted_guilds WHERE guild_id = ?").bind(i64::from(*guild_id));
    match query.execute(db).await {
        Ok(_) => (),
        Err(why) => {
            transaction.rollback().await?;

            info!("Failed to delete from RestrictedGuilds: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn insert_into(
    db: &SqlitePool,
    guild_id: &GuildId,
    reason: &String,
) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("INSERT INTO restricted_guilds (guild_id, reason) VALUES (?, ?)")
        .bind(i64::from(*guild_id))
        .bind(reason);
    match query.execute(db).await {
        Ok(_) => (),
        Err(why) => {
            let error = format!("{why}");
            if error.contains("1555") {
                // UNIQUE constraint failed
                return Ok(());
            }

            transaction.rollback().await?;

            info!("Failed to insert into RestrictedGuilds: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

    transaction.commit().await?;

    Ok(())
}
