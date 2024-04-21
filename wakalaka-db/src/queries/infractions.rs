// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, UserId};
use sqlx::{
    error::BoxDynError,
    types::chrono::{NaiveDateTime, Utc},
    Row, SqlitePool,
};
use tracing::error;
use uuid::Uuid;
use wakalaka_core::types::SqlxThrowable;

pub enum Infraction {
    Ban,
    Kick,
    Mute,
    Warn,
}

impl Infraction {
    pub fn as_str(&self) -> &str {
        match self {
            Infraction::Ban => "ban",
            Infraction::Kick => "kick",
            Infraction::Mute => "mute",
            Infraction::Warn => "warn",
        }
    }
}

pub async fn fetch_created_at_from_db(
    pool: &SqlitePool,
    uuid: &Uuid,
) -> SqlxThrowable<NaiveDateTime> {
    let query =
        sqlx::query("SELECT created_at FROM infractions WHERE uuid = ?").bind(format!("{uuid}"));

    let row = query.fetch_one(pool).await?;

    let created_at = row.get::<NaiveDateTime, _>("created_at");
    Ok(created_at)
}

pub async fn fetch_reason_from_db(pool: &SqlitePool, uuid: &Uuid) -> SqlxThrowable<String> {
    let query =
        sqlx::query("SELECT reason FROM infractions WHERE uuid = ?").bind(format!("{uuid}"));

    let row = query.fetch_one(pool).await?;

    let reason = row.get::<String, _>("reason");
    Ok(reason)
}

pub async fn fetch_moderator_id_from_db(pool: &SqlitePool, uuid: &Uuid) -> SqlxThrowable<UserId> {
    let query =
        sqlx::query("SELECT moderator_id FROM infractions WHERE uuid = ?").bind(format!("{uuid}"));

    let row = query.fetch_one(pool).await?;

    let moderator_id = UserId::from(row.get::<i64, _>("moderator_id") as u64);
    Ok(moderator_id)
}

pub async fn fetch_user_id_from_db(pool: &SqlitePool, uuid: &Uuid) -> SqlxThrowable<UserId> {
    let query =
        sqlx::query("SELECT user_id FROM infractions WHERE uuid = ?").bind(format!("{uuid}"));

    let row = query.fetch_one(pool).await?;

    let user_id = UserId::from(row.get::<i64, _>("user_id") as u64);
    Ok(user_id)
}

pub async fn fetch_guild_id_from_db(pool: &SqlitePool, uuid: &Uuid) -> SqlxThrowable<GuildId> {
    let query =
        sqlx::query("SELECT guild_id FROM infractions WHERE uuid = ?").bind(format!("{uuid}"));

    let row = query.fetch_one(pool).await?;

    let guild_id = GuildId::from(row.get::<i64, _>("guild_id") as u64);
    Ok(guild_id)
}

pub async fn fetch_kind_from_db(pool: &SqlitePool, uuid: &Uuid) -> SqlxThrowable<Infraction> {
    let query = sqlx::query("SELECT kind FROM infractions WHERE uuid = ?").bind(format!("{uuid}"));

    let row = query.fetch_one(pool).await?;

    let kind = match row.get::<String, _>("kind").as_str() {
        "ban" => Infraction::Ban,
        "kick" => Infraction::Kick,
        "mute" => Infraction::Mute,
        "warn" => Infraction::Warn,
        _ => {
            return Err(sqlx::Error::Decode(BoxDynError::from("Unknown kind")));
        }
    };
    Ok(kind)
}

pub async fn select_uuid_from_db(pool: &SqlitePool, uuid: &Uuid) -> SqlxThrowable<Uuid> {
    let query = sqlx::query("SELECT uuid FROM infractions WHERE uuid = ?").bind(format!("{uuid}"));

    let row = query.fetch_one(pool).await?;

    let uuid = Uuid::parse_str(&row.get::<String, _>("uuid")).expect("Failed to parse UUID");
    Ok(uuid)
}

pub async fn remove_infraction_from_db(pool: &SqlitePool, uuid: &Uuid) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let delete = sqlx::query("DELETE FROM infractions WHERE uuid = ?")
        .bind(format!("{uuid}"))
        .execute(pool);
    if let Err(e) = delete.await {
        error!("Failed to remove infraction from database {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn add_infraction_to_db(
    pool: &SqlitePool,
    uuid: &Uuid,
    kind: &Infraction,
    guild_id: &GuildId,
    user_id: &UserId,
    moderator_id: &UserId,
    reason: &String,
) -> SqlxThrowable<()> {
    let transaction = pool.begin().await?;

    let insert = sqlx::query(
        "INSERT INTO infractions (uuid, kind, guild_id, user_id, moderator_id, reason, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(format!("{uuid}"))
    .bind(kind.as_str())
    .bind(i64::from(*guild_id))
    .bind(i64::from(*user_id))
    .bind(i64::from(*moderator_id))
    .bind(reason)
    .bind(Utc::now());

    if let Err(e) = insert.execute(pool).await {
        error!("Failed to add infraction to database: {e:?}");

        transaction.rollback().await?;

        return Err(e.into());
    }

    transaction.commit().await?;

    Ok(())
}
