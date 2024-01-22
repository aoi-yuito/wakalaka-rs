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
use sqlx::{Row, SqlitePool};
use tokio::time::Instant;
use tracing::{debug, error};

pub(crate) async fn insert_into_guild_channels(
    channels: &Vec<GuildChannel>,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    let mut _insert_into_ok = true;

    let start_time = Instant::now();

    let transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(why) => {
            error!("Couldn't begin transaction: {why:?}");
            return Err(why);
        }
    };

    for channel in channels {
        let (channel_id, kind, guild_id, nsfw, rate_limit_per_user) = (
            channel.id,
            channel.kind,
            channel.guild_id,
            channel.nsfw,
            channel.rate_limit_per_user.unwrap_or_default(),
        );

        let query = sqlx::query(
            "INSERT INTO guild_channels (
                channel_id,
                type,
                guild_id,
                nsfw,
                rate_limit_per_user
            ) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(i64::from(channel_id))
        .bind(kind.name())
        .bind(i64::from(guild_id))
        .bind(nsfw)
        .bind(rate_limit_per_user);
        if let Err(why) = query.execute(pool).await {
            _insert_into_ok = false;

            if why.to_string().contains("1555") {
                // UNIQUE constraint failed: guild_channels.channel_id
                continue;
            }

            error!("Couldn't insert into GuildChannels: {why:?}");
            return Err(why);
        }
    }

    if let Err(why) = transaction.commit().await {
        error!("Couldn't commit transaction: {why:?}");
        return Err(why);
    }

    if _insert_into_ok {
        let elapsed_time = start_time.elapsed();
        debug!("Inserted into GuildChannels in {elapsed_time:.2?}");
    }

    Ok(())
}
