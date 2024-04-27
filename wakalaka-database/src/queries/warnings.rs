// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, UserId};
use sqlx::{
    types::chrono::{NaiveDateTime, Utc},
    PgPool,
};

use uuid::Uuid;
use wakalaka_core::types::SqlxThrowable;

pub async fn gather_all_from_db(
    pool: &PgPool,
    guild_id: &GuildId,
    user_id: &UserId,
) -> SqlxThrowable<Vec<(Uuid, UserId, UserId, String, NaiveDateTime)>> {
    let select = sqlx::query!(
        "SELECT uuid, user_id, moderator_id, reason, created_at FROM warnings WHERE guild_id = $1 AND user_id = $2",
        i64::from(*guild_id),
        i64::from(*user_id)
    );

    let rows = select.fetch_all(pool).await?;

    let warnings = rows
        .iter()
        .map(|row| {
            (
                Uuid::parse_str(&row.uuid).expect("Failed to parse UUID"),
                UserId::new(row.user_id as u64),
                UserId::new(row.moderator_id as u64),
                row.reason.clone(),
                row.created_at,
            )
        })
        .collect::<Vec<_>>();
    Ok(warnings)
}

pub async fn gather_all_uuids_from_db(
    pool: &PgPool,
    guild_id: &GuildId,
    user_id: &UserId,
) -> SqlxThrowable<Vec<Uuid>> {
    let select = sqlx::query!(
        "SELECT uuid FROM warnings WHERE guild_id = $1 AND user_id = $2",
        i64::from(*guild_id),
        i64::from(*user_id)
    );

    let rows = select.fetch_all(pool).await?;

    let uuids = rows
        .iter()
        .map(|row| Uuid::parse_str(&row.uuid).expect("Failed to parse UUID"))
        .collect();
    Ok(uuids)
}

pub async fn remove_warning_from_db(pool: &PgPool, uuid: &Uuid) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query!(
        "DELETE FROM warnings WHERE uuid = $1",
        format!("{}", uuid.simple())
    )
    .execute(pool);
    if let Err(e) = delete.await {
        tracing::error!("Failed to DELETE FROM warnings: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn add_warning_to_db(
    pool: &PgPool,
    uuid: &Uuid,
    guild_id: &GuildId,
    user_id: &UserId,
    moderator_id: &UserId,
    reason: &String,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let insert = sqlx::query!(
        "INSERT INTO warnings (uuid, guild_id, user_id, moderator_id, reason, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
        format!("{}", uuid.simple()),
        i64::from(*guild_id),
        i64::from(*user_id),
        i64::from(*moderator_id),
        reason,
        Utc::now().naive_local()
    ).execute(pool);
    if let Err(e) = insert.await {
        tracing::error!("Failed to INSERT INTO warnings: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
