// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, Timestamp, UserId};
use sqlx::{types::chrono::DateTime, PgPool, Row};
use tracing::error;
use wakalaka_core::types::SqlxThrowable;

pub async fn update_owner_id_in_db(
    pool: &PgPool,
    guild_id: &GuildId,
    owner_id: &UserId,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let update = sqlx::query("UPDATE guilds SET owner_id = $1 WHERE guild_id = ?")
        .bind(i64::from(*owner_id))
        .bind(i64::from(*guild_id))
        .execute(pool);
    if let Err(e) = update.await {
        error!("Failed to update owner ID in database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn fetch_owner_id_from_db(pool: &PgPool, guild_id: &GuildId) -> SqlxThrowable<UserId> {
    let query =
        sqlx::query("SELECT owner_id FROM guilds WHERE guild_id = $1").bind(i64::from(*guild_id));

    let row = query.fetch_one(pool).await?;

    let owner_id = UserId::from(row.get::<i64, _>("owner_id") as u64);
    Ok(owner_id)
}

pub async fn fetch_guild_id_from_db(pool: &PgPool, guild_id: &GuildId) -> SqlxThrowable<GuildId> {
    let query =
        sqlx::query("SELECT guild_id FROM guilds WHERE guild_id = $1").bind(i64::from(*guild_id));

    let row = query.fetch_one(pool).await?;

    let guild_id = GuildId::from(row.get::<i64, _>("guild_id") as u64);
    Ok(guild_id)
}

pub async fn remove_guild_from_db(pool: &PgPool, guild_id: &GuildId) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query("DELETE FROM guilds WHERE guild_id = $1")
        .bind(i64::from(*guild_id))
        .execute(pool);
    if let Err(e) = delete.await {
        error!("Failed to remove guild from database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn add_guild_to_db(
    pool: &PgPool,
    guild_id: &GuildId,
    owner_id: &UserId,
    created_at: &Timestamp,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let insert =
        sqlx::query("INSERT INTO guilds (guild_id, owner_id, created_at) VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO NOTHING")
            .bind(i64::from(*guild_id))
            .bind(i64::from(*owner_id))
            .bind(DateTime::from_timestamp(created_at.timestamp(), 0))
            .execute(pool);
    if let Err(e) = insert.await {
        error!("Failed to add guild to database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
