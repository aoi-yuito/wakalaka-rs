// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, Timestamp};
use sqlx::{types::chrono::NaiveDateTime, PgPool};

use wakalaka_core::{converters::times, types::SqlxThrowable};

pub async fn gather_all_restricted_guilds_from_db(
    pool: &PgPool,
) -> SqlxThrowable<Vec<(GuildId, String, NaiveDateTime)>> {
    let select = sqlx::query!("SELECT guild_id, reason, created_at FROM restricted_guilds");

    let mut restricted_guilds = vec![];

    let rows = select.fetch_all(pool).await?;
    for row in rows {
        let guild_id = GuildId::from(row.guild_id as u64);
        let reason = row.reason.clone();
        let created_at = row.created_at;

        restricted_guilds.push((guild_id, reason, created_at));
    }

    Ok(restricted_guilds)
}

pub async fn fetch_created_at_from_db(
    pool: &PgPool,
    guild_id: &GuildId,
) -> SqlxThrowable<NaiveDateTime> {
    let select = sqlx::query!(
        "SELECT created_at FROM restricted_guilds WHERE guild_id = $1",
        i64::from(*guild_id)
    );

    let row = select.fetch_one(pool).await?;

    let created_at = row.created_at;
    Ok(created_at)
}

pub async fn fetch_reason_from_db(pool: &PgPool, guild_id: &GuildId) -> SqlxThrowable<String> {
    let select = sqlx::query!(
        "SELECT reason FROM restricted_guilds WHERE guild_id = $1",
        i64::from(*guild_id)
    );

    let row = select.fetch_one(pool).await?;

    let reason = row.reason;
    Ok(reason)
}

pub async fn fetch_guild_id_from_db(pool: &PgPool, guild_id: &GuildId) -> SqlxThrowable<GuildId> {
    let select = sqlx::query!(
        "SELECT guild_id FROM restricted_guilds WHERE guild_id = $1",
        i64::from(*guild_id)
    );

    let row = select.fetch_one(pool).await?;

    let guild_id = GuildId::from(row.guild_id as u64);
    Ok(guild_id)
}

pub async fn remove_restricted_guild_from_db(
    pool: &PgPool,
    guild_id: &GuildId,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query!(
        "DELETE FROM restricted_guilds WHERE guild_id = $1",
        i64::from(*guild_id)
    )
    .execute(pool);
    if let Err(e) = delete.await {
        tracing::error!("Failed to DELETE FROM restricted_guilds: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn add_restricted_guild_to_db(
    pool: &PgPool,
    guild_id: &GuildId,
    reason: &String,
    created_at: &Timestamp,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let insert = sqlx::query!(
        "INSERT INTO restricted_guilds (guild_id, reason, created_at) VALUES ($1, $2, $3) ON CONFLICT (guild_id) DO NOTHING",
        i64::from(*guild_id),
        reason.trim(),
        times::datetime_to_naivedatetime(created_at)
    ).execute(pool);
    if let Err(e) = insert.await {
        tracing::error!("Failed to INSERT INTO restricted_guilds: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
