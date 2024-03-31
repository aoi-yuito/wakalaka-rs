// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ChannelId, GuildChannel, GuildId};
use std::collections::HashMap;
use wakalaka_core::types::{Context, Throwable};

pub async fn name(ctx: Context<'_>, channel_id: &ChannelId) -> Throwable<String> {
    let channel_name = channel_id.name(ctx).await?;
    Ok(channel_name)
}

pub async fn guild_channels(ctx: Context<'_>, guild_id: &GuildId) -> Throwable<Vec<GuildChannel>> {
    let channels = channels(ctx, guild_id).await?;

    let guild_channels = channels.values().cloned().collect::<Vec<_>>();
    Ok(guild_channels)
}

pub async fn channel_ids(ctx: Context<'_>, guild_id: &GuildId) -> Throwable<Vec<ChannelId>> {
    let channels = channels(ctx, guild_id).await?;

    let channel_ids = channels.keys().cloned().collect::<Vec<_>>();
    Ok(channel_ids)
}

async fn channels(
    ctx: Context<'_>,
    guild_id: &GuildId,
) -> Throwable<HashMap<ChannelId, GuildChannel>> {
    let channels = guild_id.channels(ctx).await?;
    Ok(channels)
}
