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
use tracing::error;

use crate::{database::guilds, Data};

pub async fn handle(unavailable_guild: &UnavailableGuild, guild: &Option<Guild>, data: &Data) {
    if unavailable_guild.unavailable {
        return;
    }

    let pool = &data.pool;

    let unavailable_guild_id = unavailable_guild.id;

    let guild = guild.as_ref().expect("Couldn't get guild");
    let guild_id = guild.id;

    let combined_guild_ids = vec![guild_id, unavailable_guild_id];
    for combined_guild_id in combined_guild_ids {
        if let Err(why) = guilds::delete_from_guilds(&combined_guild_id, pool).await {
            error!("Couldn't delete guild(s): {why:?}");
        }
    }
}
