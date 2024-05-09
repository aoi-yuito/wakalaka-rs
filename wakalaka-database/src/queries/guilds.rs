// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, Timestamp, UserId};
use sqlx::PgPool;

use wakalaka_core::{converters::times, types::SqlxThrowable};

pub async fn update_owner_id_in_db(
    pool: &PgPool,
    guild_id: &GuildId,
    owner_id: &UserId,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let update = sqlx::query!(
        "UPDATE guilds SET owner_id = $1 WHERE guild_id = $2",
        i64::from(*owner_id),
        i64::from(*guild_id)
    )
    .execute(pool);
    if let Err(e) = update.await {
        tracing::error!("Failed to UPDATE guilds: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn fetch_owner_id_from_db(pool: &PgPool, guild_id: &GuildId) -> SqlxThrowable<UserId> {
    let select = sqlx::query!(
        "SELECT owner_id FROM guilds WHERE guild_id = $1",
        i64::from(*guild_id)
    );

    let row = select.fetch_one(pool).await?;

    let owner_id = UserId::from(row.owner_id as u64);
    Ok(owner_id)
}

pub async fn fetch_guild_id_from_db(pool: &PgPool, guild_id: &GuildId) -> SqlxThrowable<GuildId> {
    let select = sqlx::query!(
        "SELECT guild_id FROM guilds WHERE guild_id = $1",
        i64::from(*guild_id)
    );

    let row = select.fetch_one(pool).await?;

    let guild_id = GuildId::from(row.guild_id as u64);
    Ok(guild_id)
}

pub async fn remove_guild_from_db(pool: &PgPool, guild_id: &GuildId) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query!(
        "DELETE FROM guilds WHERE guild_id = $1",
        i64::from(*guild_id)
    )
    .execute(pool);
    if let Err(e) = delete.await {
        tracing::error!("Failed to DELETE FROM guilds: {e:?}");

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

    let insert = sqlx::query!(
        "INSERT INTO guilds (guild_id, owner_id, created_at) VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO NOTHING",
        i64::from(*guild_id),
        i64::from(*owner_id),
        times::datetime_to_naivedatetime(created_at)
    ).execute(pool);
    if let Err(e) = insert.await {
        tracing::error!("Failed to INSERT INTO guilds: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
