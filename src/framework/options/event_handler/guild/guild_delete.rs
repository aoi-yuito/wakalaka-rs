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
use tracing::{error, warn};

use crate::{database::users, serenity::Context, utility, Data};

pub(crate) async fn handle_delete(
    unavailable_guild: &UnavailableGuild,
    guild: &Option<Guild>,
    ctx: &Context,
    data: &Data,
) {
    let pool = &data.pool;

    let (unavailable_guild_id, guild_id) = (
        unavailable_guild.id,
        match guild {
            Some(guild) => guild.id,
            None => {
                warn!("Couldn't get guild ID");
                return;
            }
        },
    );

    let (unavailable_members, members) = (
        utility::guilds::members_raw(ctx, unavailable_guild_id).await,
        utility::guilds::members_raw(ctx, guild_id).await,
    );
    let combined_members = unavailable_members
        .into_iter()
        .chain(members.into_iter())
        .collect::<Vec<_>>();

    users::delete_users(combined_members, pool).await;
}
