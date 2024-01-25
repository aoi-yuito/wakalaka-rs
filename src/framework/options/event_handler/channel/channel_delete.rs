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

use serenity::all::GuildChannel;
use tracing::error;

use crate::{database::restricted_guild_channels, Data};

pub async fn handle(channel: &GuildChannel, data: &Data) {
    let pool = &data.pool;

    let channel_id = channel.id;

    let previous_query =
        restricted_guild_channels::select_channel_id_from_restricted_guild_channels(&channel_id, &pool)
            .await;
    if let Ok(_) = previous_query {
        let query = restricted_guild_channels::delete_from_restricted_guild_channels(
            &channel, &pool,
        )
        .await;
        if let Err(why) = query {
            error!("Couldn't delete from RestrictedGuildChannels: {why:?}");
            return;
        }

        return;
    }
}
