
// CREATE TABLE IF NOT EXISTS violations (
//     uuid VARCHAR(32) PRIMARY KEY,
//     kind TEXT,
//     guild_id BIGINT NOT NULL,
//     user_id BIGINT NOT NULL,
//     moderator_id BIGINT NOT NULL,
//     reason VARCHAR(120) NOT NULL,
//     FOREIGN KEY (guild_id) REFERENCES guilds (guild_id),
//     FOREIGN KEY (user_id) REFERENCES users (user_id),
//     FOREIGN KEY (moderator_id) REFERENCES users (user_id)
// );

use std::borrow::Cow;

use serenity::all::{GuildId, UserId};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::SqlxError;

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

pub(crate) async fn select_uuids_from(
    db: &SqlitePool,
    kind: &Violation,
    guild_id: &GuildId,
    user_id: &UserId,
) -> Result<Vec<String>, SqlxError> {
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

pub(crate) async fn remove_from(db: &SqlitePool, uuid: &Uuid) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("DELETE FROM violations WHERE uuid = ?").bind(format!("{uuid}"));
    query.execute(db).await?;

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn insert_into(
    db: &SqlitePool,
    uuid: &Uuid,
    kind: &Violation,
    guild_id: &GuildId,
    user_id: &UserId,
    moderator_id: &UserId,
    reason: &String,
) -> Result<(), SqlxError> {
    let transaction = db.begin().await?;

    let query = sqlx::query("INSERT INTO violations (uuid, kind, guild_id, user_id, moderator_id, reason) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(format!("{uuid}"))
        .bind(Cow::from(*kind))
        .bind(i64::from(*guild_id))
        .bind(i64::from(*user_id))
        .bind(i64::from(*moderator_id))
        .bind(reason);
    query.execute(db).await?;

    transaction.commit().await?;

    Ok(())
}
