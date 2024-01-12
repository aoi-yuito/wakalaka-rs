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

use serenity::all::GuildChannel;
use sqlx::SqlitePool;
use tokio::time::Instant;
use tracing::{error, info};

pub(crate) async fn update_channels(guild_id: i64, channels: Vec<GuildChannel>, pool: &SqlitePool) {
    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Couldn't begin transaction: {why:?}");
            return;
        }
    };

    for channel in channels {
        let channel_id = i64::from(channel.id);
        let channel_kind = channel.kind.clone();
        let channel_type = channel_kind.name();
        let rate_limit_per_user = channel.rate_limit_per_user.unwrap_or(0);

        let channel_query = sqlx::query(
            "UPDATE channels SET type = ?, rate_limit_per_user = ? WHERE id = ? AND guild_id = ?",
        )
        .bind(channel_type)
        .bind(rate_limit_per_user)
        .bind(channel_id)
        .bind(guild_id);

        if let Err(why) = channel_query.execute(pool).await {
            error!("Couldn't update channel(s) in database: {why:?}");
        }
    }

    transaction.commit().await.unwrap();

    let elapsed_time = start_time.elapsed();
    info!("Updated channel(s) in database in {elapsed_time:?}");
}

pub(crate) async fn update_guilds(
    id: i64,
    owner_id: i64,
    preferred_locale: String,
    pool: &SqlitePool,
) {
    let start_time = Instant::now();

    let guild_query =
        sqlx::query("UPDATE guilds SET owner_id = ?, preferred_locale = ? WHERE id = ?")
            .bind(owner_id)
            .bind(preferred_locale)
            .bind(id);
    if let Err(why) = guild_query.execute(pool).await {
        error!("Couldn't update guild in database: {why:?}");
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Updated guild(s) in database in {elapsed_time:?}");
    }
}

pub(crate) async fn update_users(id: i64, locale: Option<String>, pool: &SqlitePool) {
    let start_time = Instant::now();

    let user_query = sqlx::query("UPDATE users SET locale = ? WHERE id = ?")
        .bind(locale)
        .bind(id);
    if let Err(why) = user_query.execute(pool).await {
        error!("Couldn't update user in database: {why:?}");
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Updated user(s) in database in {elapsed_time:?}");
    }
}

pub(crate) async fn insert_channels(guild_id: i64, channels: Vec<GuildChannel>, pool: &SqlitePool) {
    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Couldn't begin transaction: {why:?}");
            return;
        }
    };

    for channel in channels {
        let channel_id = i64::from(channel.id);
        let channel_kind = channel.kind.clone();
        let channel_type = channel_kind.name();
        let rate_limit_per_user = channel.rate_limit_per_user.unwrap_or(0);

        let channel_query = sqlx::query(
            "INSERT INTO channels (id, type, guild_id, rate_limit_per_user) VALUES (?, ?, ?, ?) ON CONFLICT (id) DO NOTHING",
        )
        .bind(channel_id)
        .bind(channel_type)
        .bind(guild_id)
        .bind(rate_limit_per_user);

        if let Err(why) = channel_query.execute(pool).await {
            error!("Couldn't insert channel(s) to database: {why:?}");
        }
    }

    transaction.commit().await.unwrap();

    let elapsed_time = start_time.elapsed();
    info!("Inserted channel(s) to database in {elapsed_time:?}");
}

pub(crate) async fn insert_guilds(
    id: i64,
    owner_id: i64,
    preferred_locale: String,
    pool: &SqlitePool,
) {
    let start_time = Instant::now();

    let guild_query = sqlx::query(
        "INSERT INTO guilds (id, owner_id, preferred_locale) VALUES (?, ?, ?) ON CONFLICT (id) DO NOTHING",
    ).bind(id).bind(owner_id).bind(preferred_locale);
    if let Err(why) = guild_query.execute(pool).await {
        error!("Couldn't insert guild to database: {why:?}");
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Inserted guild(s) to database in {elapsed_time:?}");
    }
}

pub(crate) async fn insert_users(id: i64, locale: Option<String>, pool: &SqlitePool) {
    let start_time = Instant::now();

    let user_query =
        sqlx::query("INSERT INTO users (id, locale) VALUES (?, ?) ON CONFLICT (id) DO NOTHING")
            .bind(id)
            .bind(locale);
    if let Err(why) = user_query.execute(pool).await {
        error!("Couldn't insert user to database: {why:?}");
    } else {
        let elapsed_time = start_time.elapsed();
        info!("Inserted user(s) to database in {elapsed_time:?}");
    }
}
