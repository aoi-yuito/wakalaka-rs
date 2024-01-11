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
use tracing::{error, info};

use crate::Data;

pub(crate) async fn handle(channel: &GuildChannel, data: &Data) {
    let database = &data.database;

    delete_from_channels(channel, database).await;
}

async fn delete_from_channels(channel: &GuildChannel, database: &sqlx::Pool<sqlx::Sqlite>) {
    let channel_id = i64::from(channel.id);

    let query = sqlx::query!("DELETE FROM Channels WHERE id = ?", channel_id,);
    match query.execute(database).await {
        Ok(_) => {
            info!("Deleted channel(s) from database");
        }
        Err(why) => {
            error!("Couldn't delete channel(s) from database");
            panic!("{why:?}");
        }
    }
}
