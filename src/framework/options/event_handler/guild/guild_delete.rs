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
use tracing::{error, info, warn};

use crate::{database::guilds, utility::models, Data, Error};

pub async fn handle(
    unavailable_guild: &UnavailableGuild,
    guild: &Option<Guild>,
    ctx: &crate::serenity::Context,
    data: &Data,
) -> Result<(), Error> {
    let pool = &data.pool;

    let unavailable_guild_id = unavailable_guild.id;
    let unavailable_guild_name = &models::guilds::guild_name_raw(&ctx, unavailable_guild_id);

    if unavailable_guild.unavailable {
        warn!("{unavailable_guild_name} isn't available, skipping ...");
        return Ok(());
    }

    let guild = guild.as_ref().expect("Failed to get guild");
    let (guild_id, guild_name) = (guild.id, &guild.name);

    let combined_guild_ids = vec![guild_id, unavailable_guild_id];
    for combined_guild_id in combined_guild_ids {
        if let Err(why) = guilds::delete_from_guilds(&combined_guild_id, pool).await {
            error!("Failed to delete guild(s): {why:?}");
            return Err(why.into());
        }

        let bot_name = &ctx.cache.current_user().name;

        info!("@{bot_name} left from {guild_name}");
    }

    Ok(())
}
