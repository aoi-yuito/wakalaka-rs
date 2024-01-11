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

use serenity::all::{Guild, PartialGuild};
use sqlx::{Pool, Sqlite};
use tracing::{error, info};

use crate::serenity::Context;
use crate::Data;

pub(crate) async fn handle(
    old_guild: &Option<Guild>,
    new_guild: &PartialGuild,
    ctx: &Context,
    data: &Data,
) {
    let database = &data.database;

    if let ControlFlow::Break(_) = update_guilds(old_guild, ctx, new_guild, database).await {
        return;
    }
}

async fn update_guilds(
    old_guild: &Option<Guild>,
    ctx: &Context,
    new_guild: &PartialGuild,
    database: &Pool<Sqlite>,
) -> ControlFlow<()> {
    let old_guild_id = {
        let old_guild = {
            match old_guild {
                Some(old_guild) => old_guild,
                None => {
                    return ControlFlow::Break(());
                }
            }
        };

        i64::from(old_guild.id)
    };
    let (new_guild_id, new_guild_owner_id, new_guild_unavailable) = {
        let new_guild = {
            match ctx.cache.guild(new_guild.id) {
                Some(new_guild) => new_guild,
                None => {
                    return ControlFlow::Break(());
                }
            }
        };

        let new_guild_id = i64::from(new_guild.id);
        let new_guild_owner_id = i64::from(new_guild.owner_id);
        let new_guild_unavailable = new_guild.unavailable;
        (new_guild_id, new_guild_owner_id, new_guild_unavailable)
    };
    let query = sqlx::query!(
        "UPDATE Guilds SET id = ?, ownerId = ?, isUnavailable = ? WHERE id = ?",
        new_guild_id,
        new_guild_owner_id,
        new_guild_unavailable,
        old_guild_id,
    )
    .execute(database)
    .await;

    match query {
        Ok(_) => {
            info!("Updated guild(s) in database");
        }
        Err(why) => {
            error!("Couldn't update guild(s) in database");
            panic!("{why:?}");
        }
    }
    ControlFlow::Continue(())
}
