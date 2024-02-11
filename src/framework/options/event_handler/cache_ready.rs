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
use tracing::info;

use crate::Error;

pub async fn handle(guild_ids: &Vec<GuildId>, ctx: &crate::serenity::Context) -> Result<(), Error> {
    let bot_name = &ctx.cache.current_user().name;

    let guild_count = guild_ids.len();
    if guild_count == 1 {
        info!("Prepared {guild_count} guild for @{bot_name}");
    } else {
        info!("Prepared {guild_count} guilds for @{bot_name}");
    }

    Ok(())
}
