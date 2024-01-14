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

use serenity::all::Member;
use sqlx::SqlitePool;
use tokio::time::Instant;
use tracing::{error, info};

pub(crate) async fn update_members(members: Vec<Member>, pool: &SqlitePool) {
    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Couldn't begin transaction: {why:?}");
            return;
        }
    };

    for member in members {
        if member.user.bot || member.user.system {
            continue;
        }

        let user_id = i64::from(member.user.id);
        let user_guild_id = i64::from(member.guild_id);

        let member_query = sqlx::query("UPDATE members SET guild_id = ? WHERE id = ?")
            .bind(user_guild_id)
            .bind(user_id);
        if let Err(why) = member_query.execute(pool).await {
            error!("Couldn't update member(s) in database: {why:?}");
            break;
        }
    }

    if let Err(why) = transaction.commit().await {
        error!("Couldn't commit transaction: {why:?}");
        return;
    }

    let elapsed_time = start_time.elapsed();
    info!("Updated member(s) in database in {elapsed_time:.2?}");
}

pub(crate) async fn delete_members(members: Vec<Member>, pool: &SqlitePool) {
    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Couldn't begin transaction: {why:?}");
            return;
        }
    };

    for member in members {
        if member.user.bot || member.user.system {
            continue;
        }

        let user_id = i64::from(member.user.id);

        let member_query = sqlx::query("DELETE FROM members WHERE id = ?").bind(user_id);
        if let Err(why) = member_query.execute(pool).await {
            error!("Couldn't delete member(s) from database: {why:?}");
            break;
        }
    }

    if let Err(why) = transaction.commit().await {
        error!("Couldn't commit transaction: {why:?}");
        return;
    }

    let elapsed_time = start_time.elapsed();
    info!("Deleted member(s) from database in {elapsed_time:.2?}");
}

pub(crate) async fn insert_members(members: Vec<Member>, pool: &SqlitePool) {
    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Couldn't begin transaction: {why:?}");
            return;
        }
    };

    for member in members {
        if member.user.bot || member.user.system {
            continue;
        }

        let user_id = i64::from(member.user.id);
        let user_guild_id = i64::from(member.guild_id);

        let member_query =
            sqlx::query("INSERT INTO members (id, guild_id) VALUES (?, ?) ON CONFLICT DO NOTHING")
                .bind(user_id)
                .bind(user_guild_id);
        if let Err(why) = member_query.execute(pool).await {
            error!("Couldn't insert member(s) to database: {why:?}");
            break;
        }
    }

    if let Err(why) = transaction.commit().await {
        error!("Couldn't commit transaction: {why:?}");
        return;
    }

    let elapsed_time = start_time.elapsed();
    info!("Inserted member(s) to database in {elapsed_time:.2?}");
}
