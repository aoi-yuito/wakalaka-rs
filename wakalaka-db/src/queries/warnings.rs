// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, UserId};
use sqlx::{
    types::chrono::{NaiveDateTime, Utc},
    PgPool, Row,
};

use uuid::Uuid;
use wakalaka_core::types::SqlxThrowable;

pub async fn gather_all_uuids_from_db(
    pool: &PgPool,
    guild_id: &GuildId,
    user_id: &UserId,
) -> SqlxThrowable<Vec<Uuid>> {
    let select = sqlx::query("SELECT uuid FROM warnings WHERE guild_id = $1 AND user_id = $2")
        .bind(i64::from(*guild_id))
        .bind(i64::from(*user_id));

    let rows = select.fetch_all(pool).await?;

    let uuids = rows
        .iter()
        .map(|row| Uuid::parse_str(&row.get::<String, _>("uuid")).expect("Failed to parse UUID"))
        .collect();
    Ok(uuids)
}

pub async fn gather_all_from_db(
    pool: &PgPool,
    guild_id: &GuildId,
    user_id: &UserId,
) -> SqlxThrowable<Vec<(Uuid, UserId, UserId, String, NaiveDateTime)>> {
    let select = sqlx::query(
        "SELECT uuid, user_id, moderator_id, reason, created_at FROM warnings WHERE guild_id = $1 AND user_id = $2",
    )
    .bind(i64::from(*guild_id))
    .bind(i64::from(*user_id));

    let rows = select.fetch_all(pool).await?;

    let warnings = rows
        .iter()
        .map(|row| {
            (
                Uuid::parse_str(&row.get::<String, _>("uuid")).expect("Failed to parse UUID"),
                UserId::new(row.get::<i64, _>("user_id") as u64),
                UserId::new(row.get::<i64, _>("moderator_id") as u64),
                row.get::<String, _>("reason"),
                row.get::<NaiveDateTime, _>("created_at"),
            )
        })
        .collect();
    Ok(warnings)
}

pub async fn remove_warning_from_db(pool: &PgPool, uuid: &Uuid) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query("DELETE FROM warnings WHERE uuid = $1")
        .bind(format!("{}", uuid.simple()))
        .execute(pool);
    if let Err(e) = delete.await {
        tracing::error!("Failed to remove warning from database {e:?}");

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

    let insert = sqlx::query(
        "INSERT INTO warnings (uuid, guild_id, user_id, moderator_id, reason, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(format!("{}", uuid.simple()))
    .bind(i64::from(*guild_id))
    .bind(i64::from(*user_id))
    .bind(i64::from(*moderator_id))
    .bind(reason)
    .bind(Utc::now());

    if let Err(e) = insert.execute(pool).await {
        tracing::error!("Failed to add warning to database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
