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

use serenity::all::{ChannelId, GuildChannel};
use sqlx::{Row, SqlitePool};
use tokio::time::Instant;
use tracing::{debug, error};

#[macro_export]
macro_rules! check_restricted_guild_channel {
    ($ctx:expr) => {{
        let (pool, channel_id) = (
            &$ctx.data().pool,
            crate::utility::models::channels::channel_id($ctx).await,
        );

        match crate::database::restricted_guild_channels::select_channel_id_from_restricted_guild_channels(
            &channel_id,
            pool,
        )
        .await
        {
            Ok(true) => {
                let reply = $crate::utility::components::messages::warn_reply(
                    format!("I'm afraid <#{channel_id}> is restricted."),
                    true,
                );
                if let Err(why) = $ctx.send(reply).await {
                    tracing::error!("Couldn't send reply: {why:?}");
                    return Err(why.into());
                }

                true
            },
            Ok(false) => false,
            Err(_) => {
                false
            }
        }
    }};
}

pub async fn select_channel_id_from_restricted_guild_channels(
    channel_id: &ChannelId,
    pool: &SqlitePool,
) -> Result<bool, sqlx::Error> {
    let start_time = Instant::now();

    let query = sqlx::query(
        "SELECT channel_id
        FROM restricted_guild_channels
        WHERE channel_id = ?",
    )
    .bind(i64::from(*channel_id));
    let row = match query.fetch_one(pool).await {
        Ok(row) => row,
        Err(why) => {
            return Err(why);
        }
    };

    let elapsed_time = start_time.elapsed();
    debug!("Selected from RestrictedGuildChannels in {elapsed_time:.2?}",);

    Ok(row.get::<i64, _>(0) as u64 == u64::from(*channel_id))
}

pub async fn delete_from_restricted_guild_channels(
    channel: &GuildChannel,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let channel_id = channel.id;

    let query = sqlx::query(
        "DELETE FROM restricted_guild_channels
        WHERE channel_id = ?",
    )
    .bind(i64::from(channel_id));
    if let Err(why) = query.execute(pool).await {
        error!("Couldn't delete from RestrictedGuildChannels: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Deleted from RestrictedGuildChannels in {elapsed_time:.2?}");

    Ok(())
}

pub async fn insert_into_restricted_guild_channels(
    channel: &GuildChannel,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    let channel_id = channel.id;

    let query = sqlx::query(
        "INSERT INTO restricted_guild_channels (
            channel_id
        ) VALUES (?)",
    )
    .bind(i64::from(channel_id));
    if let Err(why) = query.execute(pool).await {
        if why.to_string().contains("1555") {
            // UNIQUE constraint failed: guild_channels.channel_id
            return Ok(());
        }

        error!("Couldn't insert into RestrictedGuildChannels: {why:?}");
        return Err(why);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Inserted into RestrictedGuildChannels in {elapsed_time:.2?}");

    Ok(())
}
