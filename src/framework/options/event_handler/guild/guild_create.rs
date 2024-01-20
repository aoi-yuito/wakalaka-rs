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

use crate::{database::users, serenity::Context, utility, Data};

pub(crate) async fn handle_create(guild: &Guild, is_new: bool, ctx: &Context, data: &Data) {
    let pool = &data.pool;

    let guild_id = guild.id;

    let members = utility::guilds::members_raw(&ctx, guild_id).await;

    if !is_new {
        users::update_users(members, pool).await;
    } else {
        users::insert_users(members, pool).await;
    }
}
