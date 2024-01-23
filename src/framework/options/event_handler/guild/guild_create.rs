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
    database::{guild_members, guilds, users},
    serenity::Context,
    utility::models,
    Data,
};

pub async fn handle(guild: &Guild, is_new: bool, ctx: &Context, data: &Data) {
    if !is_new {
        return;
    }

    let pool = &data.pool;

    let guild_id = guild.id;
    let guild_members = models::members::members_raw(&ctx, &guild_id).await;

    match users::insert_into_users(&guild_members, pool).await {
        Err(why) => error!("Couldn't insert into Users: {why:?}"),
        Ok(()) => match guilds::insert_into_guilds(guild, pool).await {
            Err(why) => error!("Couldn't insert into Guilds: {why:?}"),
            Ok(()) => match guild_members::insert_into_guild_members(&guild_members, pool).await {
                Err(why) => error!("Couldn't insert into GuildMembers: {why:?}"),
                Ok(()) => (),
            },
        },
    }
}
