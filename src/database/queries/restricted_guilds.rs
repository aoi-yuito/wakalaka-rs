// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::GuildId;
use sqlx::{types::chrono::NaiveDateTime, Row, SqlitePool};
use tracing::error;
use wakalaka_core::types::SqlxThrowable;

pub async fn fetch_created_at_from_db(
    pool: &SqlitePool,
    guild_id: &GuildId,
) -> SqlxThrowable<NaiveDateTime> {
    let query = sqlx::query("SELECT created_at FROM restricted_guilds WHERE guild_id = ?")
        .bind(i64::from(*guild_id));

    let row = query.fetch_one(pool).await?;

    let created_at = row.get::<NaiveDateTime, _>("created_at");

    Ok(created_at)
}

pub async fn fetch_reason_from_db(pool: &SqlitePool, guild_id: &GuildId) -> SqlxThrowable<String> {
    let query = sqlx::query("SELECT reason FROM restricted_guilds WHERE guild_id = ?")
        .bind(i64::from(*guild_id));

    let row = query.fetch_one(pool).await?;

    let reason = row.get::<String, _>("reason");

    Ok(reason)
}

pub async fn fetch_guild_id_from_db(
    pool: &SqlitePool,
    guild_id: &GuildId,
) -> SqlxThrowable<GuildId> {
    let query = sqlx::query("SELECT guild_id FROM restricted_guilds WHERE guild_id = ?")
        .bind(i64::from(*guild_id));

    let row = query.fetch_one(pool).await?;

    let guild_id = GuildId::from(row.get::<i64, _>("guild_id") as u64);

    Ok(guild_id)
}

pub async fn remove_restricted_guild_from_db(
    pool: &SqlitePool,
    guild_id: &GuildId,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query("DELETE FROM restricted_guilds WHERE guild_id = ?")
        .bind(i64::from(*guild_id))
        .execute(pool);
    if let Err(e) = delete.await {
        error!("Failed to remove restricted guild from database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn add_restricted_guild_to_db(
    pool: &SqlitePool,
    guild_id: &GuildId,
    reason: &String,
    created_at: &NaiveDateTime,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let insert = sqlx::query(
        "INSERT INTO restricted_guilds (guild_id, reasonl, created_at) VALUES (?, ?, ?)",
    )
    .bind(i64::from(*guild_id))
    .bind(reason.trim())
    .bind(created_at)
    .execute(pool);
    if let Err(e) = insert.await {
        error!("Failed to add restricted guild to database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
