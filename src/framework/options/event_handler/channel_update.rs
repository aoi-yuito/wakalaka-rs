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
use sqlx::{Pool, Sqlite};
use tracing::{error, info};

use crate::Data;

pub(crate) async fn handle(
    old_channel: &Option<GuildChannel>,
    new_channel: &GuildChannel,
    data: &Data,
) {
    let database = &data.database;

    if let Some(old_channel) = old_channel {
        update_channels(old_channel, new_channel, database).await;
    } else {
        insert_into_channels(new_channel, database).await;
    }
}

async fn insert_into_channels(channel: &GuildChannel, database: &Pool<Sqlite>) {
    let (channel_id, parent_id, guild_id, position) = (
        i64::from(channel.id),
        channel.parent_id.map(|id| i64::from(id)),
        i64::from(channel.guild_id),
        channel.position as i64,
    );

    let query = sqlx::query!(
        "INSERT INTO Channels (id, parentId, guildId, position) VALUES (?, ?, ?, ?)",
        channel_id,
        parent_id,
        guild_id,
        position,
    );
    match query.execute(database).await {
        Ok(_) => {
            info!("Inserted channel(s) into database");
        }
        Err(why) => {
            error!("Couldn't insert channel(s) into database");
            panic!("{why:?}");
        }
    }
}

async fn update_channels(
    old_channel: &GuildChannel,
    new_channel: &GuildChannel,
    database: &Pool<Sqlite>,
) {
    let old_channel_id = i64::from(old_channel.id);
    let (channel_id, parent_id, guild_id, position) = (
        i64::from(new_channel.id),
        new_channel.parent_id.map(|id| i64::from(id)),
        i64::from(new_channel.guild_id),
        new_channel.position as i64,
    );

    let query = sqlx::query!(
        "UPDATE Channels SET id = ?, parentId = ?, guildId = ?, position = ?WHERE id = ?",
        channel_id,
        parent_id,
        guild_id,
        position,
        old_channel_id,
    );
    match query.execute(database).await {
        Ok(_) => {
            info!("Updated channel(s) in database");
        }
        Err(why) => {
            error!("Couldn't update channel(s) in database");
            panic!("{why:?}");
        }
    }
}
