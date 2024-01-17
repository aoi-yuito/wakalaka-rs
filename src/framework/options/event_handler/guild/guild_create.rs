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

use poise::serenity_prelude::Context;
use serenity::all::Guild;
use tracing::error;

use crate::{database::users, Data};

pub(crate) async fn handle_create(guild: &Guild, is_new: bool, ctx: &Context, data: &Data) {
    if !is_new {
        // The fuck does this boolean... When does ever invoke this...?
        return;
    }

    let pool = &data.pool;

    let guild_id = guild.id;

    let guild_members = match guild_id.members(&ctx.http, None, None).await {
        Ok(users) => users,
        Err(why) => {
            error!("Couldn't get guild members: {why:?}");
            return;
        }
    };

    users::insert_users(guild_members, pool).await;
}
