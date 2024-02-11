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

use crate::{database::guilds, Data, Error};

pub async fn handle(
    old_guild: &Option<Guild>,
    new_guild: &PartialGuild,
    _ctx: &crate::serenity::Context,
    data: &Data,
) -> Result<(), Error> {
    if let Some(old_guild) = old_guild {
        let new_guild_id = new_guild.id;

        let old_owner_id = old_guild.owner_id;
        let new_owner_id = new_guild.owner_id;
        if new_owner_id != old_owner_id {
            guilds::update_guilds_set_owner_id(&new_guild_id, &new_owner_id, &data.pool).await?;
        }
    }

    Ok(())
}
