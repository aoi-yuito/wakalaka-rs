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

use serenity::all::Guild;
use tracing::error;

use crate::{
    database::{guild_members, guilds, restricted_guilds, users},
    utility::models,
    Data, Error,
};

pub async fn handle(
    guild: &Guild,
    is_new: bool,
    ctx: &crate::serenity::Context,
    data: &Data,
) -> Result<(), Error> {
    let pool = &data.pool;

    if !is_new {
        return Ok(());
    }

    let guild_id = guild.id;
    let guild_name = &guild.name;
    let guild_members = match models::members::members_raw(&ctx, &guild_id).await {
        Ok(members) => members,
        Err(why) => {
            error!("Failed to get members for {guild_name}: {why:?}");
            return Err(why.into());
        }
    };

    let restricted_guild = restricted_guilds::check_restricted_guild(&pool, &guild_id).await;
    if restricted_guild {
        if let Err(why) = guild.leave(ctx).await {
            error!("Failed to leave {guild_name}: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    if users::insert_into_users(&guild_members, pool)
        .await
        .is_err()
    {
        return Ok(());
    } else if guilds::insert_into_guilds(guild, pool).await.is_err() {
        return Ok(());
    } else if guild_members::insert_into_guild_members(&guild_members, pool)
        .await
        .is_err()
    {
        return Ok(());
    }

    Ok(())
}
