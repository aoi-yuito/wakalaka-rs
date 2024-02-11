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

use serenity::all::{ChannelId, GuildId, MessageId};

use crate::{
    database::{guilds, suggestions},
    Data, Error,
};

pub async fn handle(
    channel_id: &ChannelId,
    message_id: &MessageId,
    guild_id: &Option<GuildId>,
    data: &Data,
) -> Result<(), Error> {
    let pool = &data.pool;

    if let Some(guild_id) = guild_id {
        let suggestions_channel_id =
            guilds::select_suggestions_channel_id_from_guilds(&guild_id, pool).await;
        if let Some(suggestions_channel_id) = suggestions_channel_id {
            if *channel_id == suggestions_channel_id {
                suggestions::delete_from_suggestions(
                    i64::from(*message_id),
                    i64::from(*guild_id),
                    pool,
                )
                .await;
            }
        }
    }

    Ok(())
}
