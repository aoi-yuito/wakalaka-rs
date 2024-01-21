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

use serenity::all::{Guild, PartialGuild};
use tracing::warn;

use crate::{database::users, serenity::Context, utility::models, Data};

pub(crate) async fn handle_update(
    old_guild: &Option<Guild>,
    new_guild: &PartialGuild,
    ctx: &Context,
    data: &Data,
) {
    let pool = &data.pool;

    let (old_guild_id, new_guild_id) = (
        match old_guild {
            Some(guild) => guild.id,
            None => {
                warn!("Couldn't get old guild ID");
                return;
            }
        },
        new_guild.id,
    );

    let (old_members, new_members) = (
        models::guilds::members_raw(&ctx, old_guild_id).await,
        models::guilds::members_raw(&ctx, new_guild_id).await,
    );
    let combined_members = old_members
        .into_iter()
        .chain(new_members.into_iter())
        .collect::<Vec<_>>();

    users::update_users(combined_members, pool).await;
}
