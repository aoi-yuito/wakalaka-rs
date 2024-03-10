// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::borrow::Cow;

use chrono::NaiveDateTime;
use serenity::all::{GuildId, UserId};
use sqlx::{Row, SqlitePool};
use tracing::{debug, error};

use crate::{SqlxError, SqlxThrowable};

#[derive(Copy, Clone)]
pub(crate) enum Violation {
    Warning,
    Timeout,
    Kick,
    Ban,
}

impl From<Violation> for Cow<'static, str> {
    fn from(violation: Violation) -> Cow<'static, str> {
        match violation {
            Violation::Warning => Cow::Borrowed("warning"),
            Violation::Timeout => Cow::Borrowed("timeout"),
            Violation::Kick => Cow::Borrowed("kick"),
            Violation::Ban => Cow::Borrowed("ban"),
        }
    }
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::Warning => write!(f, "warning"),
            Violation::Timeout => write!(f, "timeout"),
            Violation::Kick => write!(f, "kick"),
            Violation::Ban => write!(f, "ban"),
        }
    }
}

pub(crate) async fn select_uuids(
    db: &SqlitePool,
    kind: &Violation,
    guild_id: &GuildId,
    user_id: &UserId,
) -> SqlxThrowable<Vec<String>> {
    let query =
        sqlx::query("SELECT uuid FROM violations WHERE kind = ? AND guild_id = ? AND user_id = ?")
            .bind(Cow::from(*kind))
            .bind(i64::from(*guild_id))
            .bind(i64::from(*user_id));

    let mut uuids = vec![];

    let rows = query.fetch_all(db).await?;
    for row in rows {
        uuids.push(row.get("uuid"));
    }

    Ok(uuids)
}

pub(crate) async fn select(
    db: &SqlitePool,
    kind: &Violation,
    guild_id: &GuildId,
    user_id: &UserId,
) -> SqlxThrowable<Vec<(String, String, NaiveDateTime)>> {
    let query = sqlx::query(
        "SELECT uuid, reason, created_at FROM violations WHERE kind = ? AND guild_id = ? AND user_id = ?",
    )
    .bind(Cow::from(*kind))
    .bind(i64::from(*guild_id))
    .bind(i64::from(*user_id));

    let mut uuids = vec![];

    let rows = query.fetch_all(db).await?;
    for row in rows {
        uuids.push((row.get("uuid"), row.get("reason"), row.get("created_at")));
    }

    Ok(uuids)
}

pub(crate) async fn delete(db: &SqlitePool, uuid: &String) -> SqlxThrowable<()> {
    let transaction = db.begin().await?;

    let query = sqlx::query("DELETE FROM violations WHERE uuid = ?").bind(format!("{uuid}"));
    match query.execute(db).await {
        Ok(_) => {
            debug!("Deleted from Violations:\n\tuuid: {uuid}")
        }
        Err(why) => {
            transaction.rollback().await?;

            error!("Failed to delete from Violations: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn insert(
    db: &SqlitePool,
    uuid: &String,
    kind: &Violation,
    guild_id: &GuildId,
    user_id: &UserId,
    moderator_id: &UserId,
    reason: &String,
    created_at: &NaiveDateTime,
) -> SqlxThrowable<()> {
    let transaction = db.begin().await?;

    let query = sqlx::query("INSERT INTO violations (uuid, kind, guild_id, user_id, moderator_id, reason, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(uuid)
        .bind(Cow::from(*kind))
        .bind(i64::from(*guild_id))
        .bind(i64::from(*user_id))
        .bind(i64::from(*moderator_id))
        .bind(reason.trim())
        .bind(created_at);
    match query.execute(db).await {
        Ok(_) => {
            debug!("Inserted into Violations:\n\tuuid: {uuid}\n\tkind: {kind}\n\tguild_id: {guild_id}\n\tuser_id: {user_id}\n\tmoderator_id: {moderator_id}\n\treason: {reason}\n\tcreated_at: {created_at}");
        }
        Err(why) => {
            let error = format!("{why}");
            if error.contains("1555") {
                // UNIQUE constraint failed
                return Ok(());
            }

            transaction.rollback().await?;

            error!("Failed to insert into Violations: {why:?}");
            return Err(SqlxError::from(why));
        }
    }

    transaction.commit().await?;

    Ok(())
}
