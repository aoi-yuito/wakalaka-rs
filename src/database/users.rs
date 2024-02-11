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

pub async fn select_infractions_from_users(
    user_id: &UserId,
    pool: &SqlitePool,
) -> Result<i32, sqlx::Error> {
    let start_time = Instant::now();

    let query =
        sqlx::query("SELECT infractions FROM users WHERE user_id = ?").bind(i64::from(*user_id));
    let row = match query.fetch_one(pool).await {
        Ok(infractions) => infractions,
        Err(why) => {
            error!("Failed to select infractions from Users: {why:?}");
            return Err(why);
        }
    };

    let infractions = match row.try_get::<i32, _>("infractions") {
        Ok(infractions) => infractions,
        Err(why) => {
            error!("Failed to get infractions: {why:?}");
            return Err(why);
        }
    };

    let elapsed_time = start_time.elapsed();
    debug!("Selected infractions from Users in {elapsed_time:.2?}");

    Ok(infractions)
}

pub async fn select_user_id_from_users(user_id: &UserId, pool: &SqlitePool) -> Option<UserId> {
    let start_time = Instant::now();

    let query =
        sqlx::query("SELECT user_id FROM users WHERE user_id = ?").bind(i64::from(*user_id));
    let row = match query.fetch_one(pool).await {
        Ok(user_id) => user_id,
        Err(why) => {
            error!("Failed to select 'userId' from Users: {why:?}");
            return None;
        }
    };

    let user_id = match row.try_get::<i64, _>("user_id") {
        Ok(user_id) => user_id,
        Err(why) => {
            error!("Failed to get 'userId' from Users: {why:?}");
            return None;
        }
    };

    let elapsed_time = start_time.elapsed();
    debug!("Selected 'userId' from Users in {elapsed_time:.2?}");

    Some(UserId::from(user_id as u64))
}

pub async fn update_users_set_infractions(
    user_id: &UserId,
    infractions: i32,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query("UPDATE users SET infractions = ? WHERE user_id = ?")
        .bind(infractions)
        .bind(i64::from(*user_id));
    if let Err(why) = query.execute(pool).await {
        error!("Failed to update infractions for user(s) in Users: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated infractions for user(s) within Users in {elapsed_time:.2?}");

    Ok(())
}

pub async fn insert_into_users(
    members: &Vec<Member>,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let mut _insert_into_ok = true;

    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Failed to begin transaction: {why:?}");
            return Err(why);
        }
    };

    for member in members {
        let bot = member.user.bot;
        let system = member.user.system;
        if bot || system {
            continue;
        }

        let user_id = member.user.id;

        let query = sqlx::query(
            "INSERT INTO users (user_id) VALUES (?) ON CONFLICT(user_id) DO UPDATE SET user_id = ?",
        )
        .bind(i64::from(user_id))
        .bind(i64::from(user_id));
        if let Err(why) = query.execute(pool).await {
            _insert_into_ok = false;

            let error = format!("{why}");
            if error.contains("1555") {
                // UNIQUE constraint failed
                return Ok(());
            }

            error!("Failed to insert into Users: {why:?}");
            return Err(why);
        }
    }

    if let Err(why) = transaction.commit().await {
        error!("Failed to commit transaction: {why:?}");
        return Err(why);
    }

    if _insert_into_ok {
        let elapsed_time = start_time.elapsed();
        debug!("Inserted into Users in {elapsed_time:.2?}");
    }

    Ok(())
}
