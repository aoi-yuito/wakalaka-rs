// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ChannelId, GuildChannel, GuildId};
use tracing::warn;

use crate::{Context, Error};

pub(crate) async fn name(ctx: Context<'_>, channel_id: &ChannelId) -> String {
    match channel_id.name(ctx).await {
        Ok(channel_name) => channel_name,
        Err(_) => {
            warn!("No name from ID of channel found, using ID as name");
            format!("{channel_id}")
        }
    }
}

pub(crate) async fn channels(
    ctx: Context<'_>,
    guild_id: &GuildId,
) -> Result<Vec<GuildChannel>, Error> {
    let channel_map = guild_id.channels(ctx).await?;

    let guild_channels = channel_map.values().cloned().collect::<Vec<_>>();
    Ok(guild_channels)
}
