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

/// Disables usage of yours truly in specified channel.
#[poise::command(slash_command, owners_only)]
pub(crate) async fn restrict(
    ctx: Context<'_>,
    #[description = "Name of channel to forbid usage in."]
    #[rename = "channel"]
    channel_id: ChannelId,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(value) => value,
        None => return Ok(()),
    };

    let guild_channels = match guild_id.channels(&ctx).await {
        Ok(guild_channels) => guild_channels,
        Err(why) => {
            let message = format!("Sorry, but I couldn't get the channels for this guild: {why:?}");
            let _ = ctx.reply(message).await;

            return Ok(());
        }
    };
    for guild_channel in guild_channels {
        let first_guild_channel = guild_channel.1;
        let first_guild_channel_id = first_guild_channel.id;
        if first_guild_channel_id == channel_id {
            continue;
        }

        let restricted_channels = ctx.data().restricted_channels.read().await;
        if restricted_channels.contains(&first_guild_channel_id) {
            continue;
        }
        restricted_channels.insert(channel_id);

        let message = format!("I'm no longer able to be utilised in <#{channel_id}> anymore.");
        let _ = ctx.reply(message).await;

        return Ok(());
    }

    Ok(())
}
