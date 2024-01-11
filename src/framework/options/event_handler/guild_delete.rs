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

use serenity::all::{Guild, UnavailableGuild};
use sqlx::{Pool, Sqlite};
use tracing::{error, info};

use crate::Data;

pub(crate) async fn handle(
    unavailable_guild: &UnavailableGuild,
    guild: &Option<Guild>,
    data: &Data,
) {
    let database = &data.database;

    if let Some(guild) = guild {
        delete_from_guilds(guild, database).await;
    } else {
        delete_from_guilds_unavailable(unavailable_guild, database).await;
    }
}

async fn delete_from_guilds_unavailable(
    unavailable_guild: &UnavailableGuild,
    database: &Pool<Sqlite>,
) {
    let guild_id = i64::from(unavailable_guild.id);
    let guild_unavailable = unavailable_guild.unavailable;

    let query = sqlx::query!(
        "DELETE FROM Guilds WHERE id = ? AND isUnavailable = ?",
        guild_id,
        guild_unavailable,
    );
    match query.execute(database).await {
        Ok(_) => {
            info!("Deleted guild(s) from database");
        }
        Err(why) => {
            error!("Couldn't delete guild(s) from database");
            panic!("{why:?}");
        }
    }
}

async fn delete_from_guilds(guild: &Guild, database: &Pool<Sqlite>) {
    let guild_id = i64::from(guild.id);
    let guild_owner_id = i64::from(guild.owner_id);
    let guild_preferred_locale = guild.preferred_locale.clone();
    let guild_unavailable = guild.unavailable;

    let query = sqlx::query!(
        "DELETE FROM Guilds WHERE id = ? AND ownerId = ? AND preferredLocale = ? AND isUnavailable = ?",
        guild_id,
        guild_owner_id,
        guild_preferred_locale,
        guild_unavailable,
    );
    match query.execute(database).await {
        Ok(_) => {
            info!("Inserted guild(s) into database");
        }
        Err(why) => {
            error!("Couldn't insert guild(s) into database");
            panic!("{why:?}");
        }
    }
}
