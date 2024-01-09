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

use crate::{check_administrator_permission, Context, Error};

/// Enables usage of yours truly in specified channel.
#[poise::command(slash_command)]
pub(crate) async fn unrestrict(
    ctx: Context<'_>,
    #[description = "Name of channel to restrict usage in."]
    #[rename = "channel"]
    channel_id: ChannelId,
) -> Result<(), Error> {
    check_administrator_permission!(ctx);

    let restricted_channels = ctx.data().restricted_channels.write().await;
    restricted_channels.remove(&channel_id);

    let message = format!("I'm able to be utilised in <#{channel_id}> again.");
    let _ = ctx.reply(message).await;

    Ok(())
}
