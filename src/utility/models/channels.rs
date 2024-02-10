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

use serenity::{
    all::{ChannelId, GuildChannel, Mention, Mentionable},
    model::ModelError,
};
use tracing::error;

use crate::{Context, Error};

use super::guilds;

pub async fn channel_mention_from_channel_id(channel_id: ChannelId) -> Result<Mention, Error> {
    Ok(channel_id.mention())
}

pub async fn channel_name_from_channel_id(
    ctx: Context<'_>,
    channel_id: ChannelId,
) -> Result<String, Error> {
    Ok(channel_id.name(ctx).await?)
}

pub async fn channel_name(ctx: Context<'_>) -> Result<String, ModelError> {
    Ok(channel(ctx).await?.name)
}

pub fn channel_id(ctx: Context<'_>) -> ChannelId {
    ctx.channel_id()
}

pub async fn channels(ctx: Context<'_>) -> Result<Vec<GuildChannel>, ModelError> {
    let channels = match guilds::guild(ctx)?.channels(ctx).await {
        Ok(channels) => channels.values().cloned().collect::<Vec<GuildChannel>>(),
        Err(why) => {
            error!("Couldn't get channels: {why:?}");
            return Err(ModelError::ChannelNotFound);
        }
    };
    Ok(channels)
}

pub async fn channel_from_channel_id_raw(
    ctx: &crate::serenity::Context,
    channel_id: &ChannelId,
) -> Result<GuildChannel, ModelError> {
    let channel = match channel_id.to_channel(ctx).await {
        Ok(channel) => channel.guild().ok_or(ModelError::ChannelNotFound)?,
        Err(why) => {
            error!("Couldn't get channel: {why:?}");
            return Err(ModelError::ChannelNotFound);
        }
    };
    Ok(channel)
}

pub async fn channel(ctx: Context<'_>) -> Result<GuildChannel, ModelError> {
    let channel = match channel_id(ctx).to_channel(ctx).await {
        Ok(channel) => channel.guild().ok_or(ModelError::ChannelNotFound)?,
        Err(why) => {
            error!("Couldn't get channel: {why:?}");
            return Err(ModelError::ChannelNotFound);
        }
    };
    Ok(channel)
}
