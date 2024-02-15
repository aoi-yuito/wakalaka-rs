
use serenity::all::{GuildId, UserId};
use sqlx::{Row, SqlitePool};

use crate::SqlxError;

pub(crate) async fn select_guild_id_from(
    db: &SqlitePool,
    guild_id: &GuildId,
) -> Result<GuildId, SqlxError> {
    let query =
        sqlx::query("SELECT guild_id FROM guilds WHERE guild_id = ?").bind(i64::from(*guild_id));

    let row = query.fetch_one(db).await?;

    let guild_id = GuildId::from(row.get::<i64, _>("guild_id") as u64);
    Ok(guild_id)
}

pub(crate) async fn update_set_owner_id(
    db: &SqlitePool,
    guild_id: &GuildId,
    owner_id: &UserId,
) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("UPDATE guilds SET owner_id = ? WHERE guild_id = ?")
        .bind(i64::from(*owner_id))
        .bind(i64::from(*guild_id));
    query.execute(db).await?;

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn delete_from(db: &SqlitePool, guild_id: &GuildId) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("DELETE FROM guilds WHERE guild_id = ?").bind(i64::from(*guild_id));
    query.execute(db).await?;

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn insert_into(
    db: &SqlitePool,
    guild_id: &GuildId,
    owner_id: &UserId,
    reason: &String,
) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("INSERT INTO guilds (guild_id, owner_id, reason) VALUES (?, ?, ?)")
        .bind(i64::from(*guild_id))
        .bind(i64::from(*owner_id))
        .bind(reason);
    query.execute(db).await?;

    transaction.commit().await?;

    Ok(())
}
