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
use tracing::{error, warn};

use crate::{database::suggestions, serenity::Context, Data};

pub(crate) async fn handle_delete(
    channel_id: &ChannelId,
    message_id: &MessageId,
    guild_id: &Option<GuildId>,
    ctx: &Context,
    data: &Data,
) {
    let pool = &data.pool;

    let channel = match channel_id.to_channel(&ctx).await {
        Ok(value) => value,
        Err(why) => {
            error!("Couldn't get channel: {why:?}");
            return;
        }
    };
    let channel_id = channel.id();
    let channel_name = match channel_id.name(&ctx.http).await {
        Ok(value) => value,
        Err(why) => {
            error!("Couldn't get channel name: {why:?}");
            return;
        }
    };
    if channel_name == "suggestions" {
        let (message_id, guild_id) = (
            i64::from(*message_id),
            match guild_id {
                Some(value) => i64::from(*value),
                None => {
                    warn!("Couldn't get guild ID");
                    return;
                }
            },
        );

        suggestions::delete_suggest(message_id, guild_id, pool).await;
        return;
    }
}
