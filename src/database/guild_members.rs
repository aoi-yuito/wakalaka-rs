// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

use serenity::all::{Member, UserId};
use sqlx::{Row, SqlitePool};
use tokio::time::Instant;
use tracing::{debug, error};

pub(crate) async fn update_guilds_members_set_deaf(
    user_id: &UserId,
    deaf: bool,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "UPDATE guild_members
        SET deaf = ?
        WHERE user_id = ?",
    )
    .bind(deaf)
    .bind(i64::from(*user_id));
    if let Err(why) = query.execute(pool).await {
        error!("Couldn't update 'deaf' from GuildMembers: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated 'deaf' from GuildMembers in {elapsed_time:.2?}");

    Ok(())
}

pub(crate) async fn update_guilds_members_set_mute(
    user_id: &UserId,
    mute: bool,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "UPDATE guild_members
        SET mute = ?
        WHERE user_id = ?",
    )
    .bind(mute)
    .bind(i64::from(*user_id));
    if let Err(why) = query.execute(pool).await {
        error!("Couldn't update 'mute' from GuildMembers: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated 'mute' from GuildMembers in {elapsed_time:.2?}");

    Ok(())
}

pub(crate) async fn update_guilds_members_set_timeout(
    user_id: &UserId,
    timeout: bool,
    communication_disabled_until: Option<String>,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "UPDATE guild_members
        SET timeout = ?, communication_disabled_until = ?
        WHERE user_id = ?",
    )
    .bind(timeout)
    .bind(communication_disabled_until)
    .bind(i64::from(*user_id));
    if let Err(why) = query.execute(pool).await {
        error!("Couldn't update 'timeout' from GuildMembers: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated 'timeout' from GuildMembers in {elapsed_time:.2?}");

    Ok(())
}

pub(crate) async fn update_guilds_members_set_ban(
    user_id: &UserId,
    ban: bool,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "UPDATE guild_members
        SET ban = ?
        WHERE user_id = ?",
    )
    .bind(ban)
    .bind(i64::from(*user_id));
    if let Err(why) = query.execute(pool).await {
        error!("Couldn't update 'ban' from GuildMembers: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated 'ban' from GuildMembers in {elapsed_time:.2?}");

    Ok(())
}

pub(crate) async fn insert_into_guild_members(
    members: &Vec<Member>,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Couldn't begin transaction: {why:?}");
            return Err(why);
        }
    };

    for member in members {
        let (bot, system) = (member.user.bot, member.user.system);
        if bot || system {
            continue;
        }

        let (user_id, deaf, mute, guild_id) =
            (member.user.id, member.deaf, member.mute, member.guild_id);

        // This query avoids inserting duplicate rows to prevent "UNIQUE constraint failed" error.
        let existing_query =
            sqlx::query("SELECT user_id FROM guild_members WHERE user_id = ? AND guild_id = ?")
                .bind(i64::from(user_id))
                .bind(i64::from(guild_id));
        let existing_row = match existing_query.fetch_one(pool).await {
            Ok(existing_row) => existing_row,
            Err(why) => {
                error!("Couldn't select from GuildMembers: {why:?}");
                return Err(why);
            }
        };
        if let Err(why) = existing_row.try_get::<i64, _>("user_id") {
            error!("Couldn't get 'userId' from GuildMembers: {why:?}");
            return Err(why);
        } else if existing_row.try_get::<i64, _>("user_id")? == i64::from(user_id) {
            continue;
        }

        let query = sqlx::query(
            "INSERT INTO guild_members (
            user_id,
            deaf,
            mute,
            timeout,
            ban,
            guild_id
        ) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(i64::from(user_id))
        .bind(deaf)
        .bind(mute)
        .bind(false)
        .bind(false)
        .bind(i64::from(guild_id));
        if let Err(why) = query.execute(pool).await {
            error!("Couldn't insert into GuildMembers: {why:?}");
            return Err(why);
        }
    }

    if let Err(why) = transaction.commit().await {
        error!("Couldn't commit transaction: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Inserted into GuildMembers in {elapsed_time:.2?}");

    Ok(())
}
