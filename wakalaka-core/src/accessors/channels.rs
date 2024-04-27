// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::types::{Context, SContext, Throwable};
use serenity::all::{ChannelId, GuildChannel, GuildId};
use std::collections::HashMap;

pub async fn fetch_raw_channel_name_from_id(
    ctx: &SContext,
    channel_id: &ChannelId,
) -> Throwable<String> {
    let channel_name = channel_id.name(ctx).await?;
    Ok(channel_name)
}

pub async fn fetch_channel_name_from_id(
    ctx: Context<'_>,
    channel_id: &ChannelId,
) -> Throwable<String> {
    let channel_name = channel_id.name(ctx).await?;
    Ok(channel_name)
}

pub async fn gather_all_guild_channels(
    ctx: Context<'_>,
    guild_id: &GuildId,
) -> Throwable<Vec<GuildChannel>> {
    let channels = fetch_channels_from_guild(ctx, guild_id).await?;

    let guild_channels = channels.values().cloned().collect::<Vec<_>>();
    Ok(guild_channels)
}

pub async fn gather_all_channel_ids(
    ctx: Context<'_>,
    guild_id: &GuildId,
) -> Throwable<Vec<ChannelId>> {
    let channels = fetch_channels_from_guild(ctx, guild_id).await?;

    let channel_ids = channels.keys().cloned().collect::<Vec<_>>();
    Ok(channel_ids)
}

async fn fetch_channels_from_guild(
    ctx: Context<'_>,
    guild_id: &GuildId,
) -> Throwable<HashMap<ChannelId, GuildChannel>> {
    let channels = guild_id.channels(ctx).await?;
    Ok(channels)
}
