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

use chrono::NaiveDateTime;
use serenity::all::Guild;
use tracing::{error, info};

use crate::{serenity::Context, Data};

pub(crate) async fn handle(guild: &Guild, is_new: bool, ctx: &Context, data: &Data) {
    if !is_new {
        return;
    }

    let database = &data.database;

    let (
        guild_id,
        guild_name,
        guild_owner_id,
        guild_joined_at,
        guild_large,
        guild_unavailable,
    ) = {
        let guild = {
            match ctx.cache.guild(guild.id) {
                Some(guild) => guild,
                None => {
                    error!("Couldn't get guild from cache");
                    return;
                }
            }
        };

        let guild_id = i64::from(guild.id);
        let guild_name = guild.name.clone();
        let guild_owner_id = i64::from(guild.owner_id);
        let guild_joined_at =
            match NaiveDateTime::from_timestamp_opt(guild.joined_at.timestamp(), 0) {
                Some(joined_at) => joined_at,
                None => {
                    error!("Couldn't convert guild join date");
                    return;
                }
            };
        let guild_large = guild.large;
        let guild_unavailable = guild.unavailable;

        (
            guild_id,
            guild_name,
            guild_owner_id,
            guild_joined_at,
            guild_large,
            guild_unavailable,
        )
    };

    let query = sqlx::query!(
        "INSERT INTO guilds (id, name, owner_id, joined_at, large, unavailable) VALUES (?, ?, ?, ?, ?, ?)",
        guild_id,
        guild_name,
        guild_owner_id,
        guild_joined_at,
        guild_large,
        guild_unavailable,
    ).execute(database).await;
    match query {
        Ok(_) => {
            info!("Inserted guild into database");
        }
        Err(why) => {
            error!("Couldn't insert guild into database");
            panic!("{why:?}");
        }
    }
}
