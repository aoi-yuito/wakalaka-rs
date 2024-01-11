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

use std::ops::ControlFlow;

use serenity::all::Guild;
use sqlx::{Pool, Sqlite};
use tracing::{error, info};

use crate::{serenity::Context, Data};

pub(crate) async fn handle(guild: &Guild, is_new: bool, ctx: &Context, data: &Data) {
    // if !is_new {
    //     return;
    // }

    let database = &data.database;

    if let ControlFlow::Break(_) = insert_into_guilds(ctx, guild, database).await {
        return;
    }
}

async fn insert_into_guilds(
    ctx: &Context,
    guild: &Guild,
    database: &Pool<Sqlite>,
) -> ControlFlow<()> {
    let (guild_id, guild_owner_id, guild_unavailable) = {
        let guild = {
            match ctx.cache.guild(guild.id) {
                Some(guild) => guild,
                None => {
                    error!("Couldn't get guild from cache");
                    return ControlFlow::Break(());
                }
            }
        };

        let guild_id = i64::from(guild.id);
        let guild_owner_id = i64::from(guild.owner_id);
        let guild_unavailable = guild.unavailable;
        (guild_id, guild_owner_id, guild_unavailable)
    };
    let query = sqlx::query!(
        "INSERT INTO Guilds (id, ownerId, isUnavailable) VALUES (?, ?, ?)",
        guild_id,
        guild_owner_id,
        guild_unavailable,
    )
    .execute(database)
    .await;

    match query {
        Ok(_) => {
            info!("Inserted guild(s) into database");
        }
        Err(why) => {
            error!("Couldn't insert guild(s) into database");
            panic!("{why:?}");
        }
    }
    ControlFlow::Continue(())
}
