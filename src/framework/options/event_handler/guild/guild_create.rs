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

use crate::{
    check_restricted_guild,
    database::{guild_members, guilds, users},
    utility::models,
    Data,
};

pub async fn handle(guild: &Guild, is_new: bool, ctx: &crate::serenity::Context, data: &Data) {
    let pool = &data.pool;

    let guild_id = guild.id;
    let guild_members = match models::members::members_raw(&ctx, &guild_id).await {
        Ok(members) => members,
        Err(_) => {
            return;
        }
    };

    let restricted_guild = check_restricted_guild!(&pool, &guild_id);
    if restricted_guild || !is_new {
        return;
    }

    if users::insert_into_users(&guild_members, pool)
        .await
        .is_err()
    {
        return;
    } else if guilds::insert_into_guilds(guild, pool).await.is_err() {
        return;
    } else if guild_members::insert_into_guild_members(&guild_members, pool)
        .await
        .is_err()
    {
        return;
    }
}
