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

use serenity::all::{ChannelId, Guild, GuildId, UserId};
use sqlx::{Row, SqlitePool};
use tokio::time::Instant;
use tracing::{debug, error};

pub async fn check_logs_channel(guild_id: &GuildId, pool: &SqlitePool) -> Option<ChannelId> {
    select_logs_channel_id_from_guilds(guild_id, pool).await
}

pub async fn check_welcome_channel(guild_id: &GuildId, pool: &SqlitePool) -> Option<ChannelId> {
    select_welcome_channel_id_from_guilds(guild_id, pool).await
}

pub async fn select_logs_channel_id_from_guilds(
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Option<ChannelId> {
    let start_time = Instant::now();

    let query = sqlx::query("SELECT logs_channel_id FROM guilds WHERE guild_id = ?")
        .bind(i64::from(*guild_id));
    let row = match query.fetch_one(pool).await {
        Ok(row) => row,
        Err(_) => {
            return None;
        }
    };

    let logs_channel_id = match row.try_get::<i64, _>("logs_channel_id") {
        Ok(logs_channel_id) => logs_channel_id,
        Err(why) => {
            error!("Failed to get 'logsChannelId' from Guilds: {why:?}");
            return None;
        }
    };
    if logs_channel_id != 0 {
        let elapsed_time = start_time.elapsed();
        debug!("Selected 'logsChannelId' from Guilds in {elapsed_time:.2?}");

        return Some(ChannelId::from(logs_channel_id as u64));
    }

    None
}

pub async fn select_welcome_channel_id_from_guilds(
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Option<ChannelId> {
    let start_time = Instant::now();

    let query = sqlx::query("SELECT welcome_channel_id FROM guilds WHERE guild_id = ?")
        .bind(i64::from(*guild_id));
    let row = match query.fetch_one(pool).await {
        Ok(row) => row,
        Err(_) => {
            return None;
        }
    };

    let welcome_channel_id = match row.try_get::<i64, _>("welcome_channel_id") {
        Ok(welcome_channel_id) => welcome_channel_id,
        Err(why) => {
            error!("Failed to get 'welcomeChannelId' from Guilds: {why:?}");
            return None;
        }
    };
    if welcome_channel_id != 0 {
        let elapsed_time = start_time.elapsed();
        debug!("Selected 'welcomeChannelId' from Guilds in {elapsed_time:.2?}");

        return Some(ChannelId::from(welcome_channel_id as u64));
    }

    None
}

pub async fn select_suggestions_channel_id_from_guilds(
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Option<ChannelId> {
    let start_time = Instant::now();

    let query = sqlx::query("SELECT suggestions_channel_id FROM guilds WHERE guild_id = ?")
        .bind(i64::from(*guild_id));
    let row = match query.fetch_one(pool).await {
        Ok(row) => row,
        Err(_) => {
            return None;
        }
    };

    let suggestions_channel_id = match row.try_get::<i64, _>("suggestions_channel_id") {
        Ok(suggestions_channel_id) => suggestions_channel_id,
        Err(why) => {
            error!("Failed to get 'suggestionsChannelId' from Guilds: {why:?}");
            return None;
        }
    };
    if suggestions_channel_id != 0 {
        let elapsed_time = start_time.elapsed();
        debug!("Selected 'suggestionsChannelId' from Guilds in {elapsed_time:.2?}");

        return Some(ChannelId::from(suggestions_channel_id as u64));
    }

    None
}

pub async fn select_usage_channel_id_from_guilds(
    guild_id: &GuildId,
    pool: &SqlitePool,
) -> Option<ChannelId> {
    let start_time = Instant::now();

    let query = sqlx::query("SELECT usage_channel_id FROM guilds WHERE guild_id = ?")
        .bind(i64::from(*guild_id));
    let row = match query.fetch_one(pool).await {
        Ok(row) => row,
        Err(_) => {
            return None;
        }
    };

    let usage_channel_id = match row.try_get::<i64, _>("usage_channel_id") {
        Ok(usage_channel_id) => usage_channel_id,
        Err(why) => {
            error!("Failed to get 'usageChannelId' from Guilds: {why:?}");
            return None;
        }
    };
    if usage_channel_id != 0 {
        let elapsed_time = start_time.elapsed();
        debug!("Selected 'usageChannelId' from Guilds in {elapsed_time:.2?}");

        return Some(ChannelId::from(usage_channel_id as u64));
    }

    None
}

pub async fn select_guild_id_from_guilds(guild_id: &GuildId, pool: &SqlitePool) -> Option<GuildId> {
    let start_time = Instant::now();

    let query =
        sqlx::query("SELECT guild_id FROM guilds WHERE guild_id = ?").bind(i64::from(*guild_id));
    let row = match query.fetch_one(pool).await {
        Ok(row) => row,
        Err(_) => {
            return None;
        }
    };

    let guild_id = match row.try_get::<i64, _>("guild_id") {
        Ok(guild_id) => guild_id,
        Err(why) => {
            error!("Failed to get 'guildId' from Guilds: {why:?}");
            return None;
        }
    };
    if guild_id != 0 {
        let elapsed_time = start_time.elapsed();
        debug!("Selected 'guildId' from Guilds in {elapsed_time:.2?}");

        return Some(GuildId::from(guild_id as u64));
    }

    None
}

pub async fn update_guilds_set_logs_channel_id(
    channel_id: ChannelId,
    guild_id: GuildId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query("UPDATE guilds SET logs_channel_id = ? WHERE guild_id = ?")
        .bind(i64::from(channel_id))
        .bind(i64::from(guild_id));
    if let Err(why) = query.execute(pool).await {
        error!("Failed to update 'logsChannelId' from Guilds: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated 'logsChannelId' from Guilds in {elapsed_time:.2?}");

    Ok(())
}

pub async fn update_guilds_set_welcome_channel_id(
    channel_id: ChannelId,
    guild_id: GuildId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query("UPDATE guilds SET welcome_channel_id = ? WHERE guild_id = ?")
        .bind(i64::from(channel_id))
        .bind(i64::from(guild_id));
    if let Err(why) = query.execute(pool).await {
        error!("Failed to update 'welcomeChannelId' from Guilds: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated 'welcomeChannelId' from Guilds in {elapsed_time:.2?}");

    Ok(())
}

pub async fn update_guilds_set_suggestions_channel_id(
    channel_id: ChannelId,
    guild_id: GuildId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query("UPDATE guilds SET suggestions_channel_id = ? WHERE guild_id = ?")
        .bind(i64::from(channel_id))
        .bind(i64::from(guild_id));
    if let Err(why) = query.execute(pool).await {
        error!("Failed to update 'suggestionsChannelId' from Guilds: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated 'suggestionsChannelId' from Guilds in {elapsed_time:.2?}");

    Ok(())
}

pub async fn update_guilds_set_usage_channel_id(
    channel_id: ChannelId,
    guild_id: GuildId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query("UPDATE guilds SET usage_channel_id = ? WHERE guild_id = ?")
        .bind(i64::from(channel_id))
        .bind(i64::from(guild_id));
    if let Err(why) = query.execute(pool).await {
        error!("Failed to update 'usageChannelId' from Guilds: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated 'usageChannelId' from Guilds in {elapsed_time:.2?}");

    Ok(())
}

pub async fn update_guilds_set_owner_id(
    guild_id: &GuildId,
    owner_id: &UserId,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Failed to begin transaction: {why:?}");
            return Err(why);
        }
    };

    let query = sqlx::query(
        "UPDATE guilds SET
        owner_id = ?
        WHERE guild_id = ?",
    )
    .bind(i64::from(*owner_id))
    .bind(i64::from(*guild_id));
    if let Err(why) = query.execute(pool).await {
        error!("Failed to update Guilds: {why:?}");
        return Err(why);
    }

    if let Err(why) = transaction.commit().await {
        error!("Failed to commit transaction: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Updated Guilds in {elapsed_time:.2?}");

    Ok(())
}

pub async fn delete_from_guilds(guild_id: &GuildId, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Failed to begin transaction: {why:?}");
            return Err(why);
        }
    };

    let query = sqlx::query("DELETE FROM guilds WHERE guild_id = ?").bind(i64::from(*guild_id));
    if let Err(why) = query.execute(pool).await {
        error!("Failed to delete Guilds: {why:?}");
        return Err(why);
    }

    if let Err(why) = transaction.commit().await {
        error!("Failed to commit transaction: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Deleted Guilds in {elapsed_time:.2?}");

    Ok(())
}

pub async fn insert_into_guilds(guild: &Guild, pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut _insert_into_ok = true;

    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Failed to begin transaction: {why:?}");
            return Err(why);
        }
    };

    let guild_id = guild.id;
    let owner_id = guild.owner_id;

    let query = sqlx::query(
        "INSERT INTO guilds (
            guild_id,
            owner_id
        ) VALUES (?, ?)",
    )
    .bind(i64::from(guild_id))
    .bind(i64::from(owner_id));
    if let Err(why) = query.execute(pool).await {
        _insert_into_ok = false;

        if why.to_string().contains("1555") {
            // UNIQUE constraint failed
            return Ok(());
        }

        error!("Failed to insert into Guilds: {why:?}");
        return Err(why);
    }

    if let Err(why) = transaction.commit().await {
        error!("Failed to commit transaction: {why:?}");
        return Err(why);
    }

    if _insert_into_ok {
        let elapsed_time = start_time.elapsed();
        debug!("Inserted into Guilds in {elapsed_time:.2?}");
    }

    Ok(())
}
