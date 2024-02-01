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

use serenity::all::GuildId;
use tracing::{error, info};

use crate::{check_restricted_guild, serenity::Context, utility::models, Data};

pub async fn handle(guild_ids: &Vec<GuildId>, ctx: &Context, data: &Data) {
    let pool = &data.pool;

    for guild_id in guild_ids {
        let guild_name = models::guilds::guild_name_from_guild_id_raw(ctx, *guild_id);

        let restricted_guild = check_restricted_guild!(&pool, &guild_id);
        if restricted_guild {
            if let Err(why) = guild_id.leave(ctx).await {
                error!("Couldn't leave {guild_name}: {why:?}");
            }

            return;
        }
    }

    let bot_name = models::current_application_name_raw(ctx).await.unwrap();

    let guild_count = guild_ids.len();

    info!("Prepared {guild_count} guild(s) for @{bot_name}");
}
