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

use serenity::all::UserId;
use sqlx::{Row, SqlitePool};
use tokio::time::Instant;
use tracing::{debug, error};

use crate::{utility::components::messages, Context};

pub async fn check_restricted_user(ctx: Context<'_>) -> bool {
    let (pool, user_id) = (&ctx.data().pool, ctx.author().id);

    match select_user_id_from_restricted_users(&user_id, pool).await {
        Ok(true) => {
            let reply = messages::error_reply(
                format!("Sorry, but you can't use yours truly anymore.\n\nIf you think this is a mistake, contact the [developer](https://github.com/Kawaxte) on GitHub, or swing by the [support server](https://discord.gg/jUZVWk7q2q) for help.\n\nIn the meantime, take a moment to think about what went down, because this can't be undone."), true);
            ctx.send(reply).await.unwrap();

            true
        }
        Ok(false) => false,
        Err(_) => false,
    }
}

pub async fn select_user_id_from_restricted_users(
    user_id: &UserId,
    pool: &SqlitePool,
) -> Result<bool, sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query("SELECT user_id FROM restricted_users WHERE user_id = ?")
        .bind(i64::from(*user_id));

    let row = match query.fetch_one(pool).await {
        Ok(row) => row,
        Err(why) => {
            return Err(why);
        }
    };

    let elapsed_time = start_time.elapsed();
    debug!("Selected from RestrictedUsers in {elapsed_time:.2?}",);

    Ok(row.get::<i64, _>(0) as u64 == u64::from(*user_id))
}

pub async fn delete_from_restricted_users(
    user_id: &UserId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query =
        sqlx::query("DELETE FROM restricted_users WHERE user_id = ?").bind(i64::from(*user_id));
    if let Err(why) = query.execute(pool).await {
        error!("Failed to delete from RestrictedUsers: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Deleted from RestrictedUsers in {elapsed_time:.2?}");

    Ok(())
}

pub async fn insert_into_restricted_users(
    user_id: &UserId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query =
        sqlx::query("INSERT INTO restricted_users (user_id) VALUES (?)").bind(i64::from(*user_id));
    if let Err(why) = query.execute(pool).await {
        if why.to_string().contains("1555") {
            // UNIQUE constraint failed
            return Ok(());
        }

        error!("Failed to insert into RestrictedUsers: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Inserted into RestrictedUsers in {elapsed_time:.2?}");

    Ok(())
}
