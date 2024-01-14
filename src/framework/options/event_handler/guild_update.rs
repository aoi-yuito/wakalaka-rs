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
use serenity::all::{Guild, PartialGuild};
use tracing::{error, warn};

use crate::{database::members, Data};

pub(crate) async fn handle_update(
    old_guild: &Option<Guild>,
    new_guild: &PartialGuild,
    ctx: &Context,
    data: &Data,
) {
    let pool = &data.pool;

    let old_guild_id = match old_guild {
        Some(guild) => guild.id,
        None => {
            warn!("Couldn't get old guild ID");
            return;
        }
    };
    let new_guild_id = new_guild.id;

    let old_members = match old_guild_id.members(&ctx.http, None, None).await {
        Ok(users) => users,
        Err(why) => {
            error!("Couldn't get old guild members: {why:?}");
            return;
        }
    };
    let new_members = match new_guild_id.members(&ctx.http, None, None).await {
        Ok(users) => users,
        Err(why) => {
            error!("Couldn't get new guild members: {why:?}");
            return;
        }
    };

    let guild_members = old_members
        .into_iter()
        .chain(new_members.into_iter())
        .collect::<Vec<_>>();

    members::update_members(guild_members, pool).await;
}
