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

use serenity::all::ChannelId;

use crate::{Context, Error};

/// Enables usage of yours truly in provided channel.
#[poise::command(slash_command, owners_only)]
pub(crate) async fn unrestrict(
    ctx: Context<'_>,
    #[description = "Channel to unrestrain usage in."]
    #[rename = "channel"]
    channel_id: ChannelId,
) -> Result<(), Error> {
    let restricted_channels = ctx.data().restricted_channels.write().await;
    restricted_channels.remove(&channel_id);

    let message = format!("Alright, I'll respond to commands in <#{channel_id}> again.");
    let _ = ctx.reply(message).await;

    Ok(())
}
