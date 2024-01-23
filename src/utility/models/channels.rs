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

use serenity::all::{ChannelId, GuildChannel, GuildId};
use tracing::error;

use crate::Context;

use super::guilds;

pub async fn channel_id(ctx: Context<'_>) -> ChannelId {
    ctx.channel_id()
}

pub async fn channels_raw(ctx: &crate::serenity::Context, guild_id: GuildId) -> Vec<GuildChannel> {
    match guilds::guild_raw(ctx, guild_id).channels(&ctx).await {
        Ok(channels) => channels.values().cloned().collect::<Vec<GuildChannel>>(),
        Err(why) => {
            error!("Couldn't get channels: {why:?}");
            return Vec::new();
        }
    }
}

pub async fn channels(ctx: Context<'_>) -> Vec<GuildChannel> {
    match guilds::guild(ctx).await.channels(&ctx).await {
        Ok(channels) => channels.values().cloned().collect::<Vec<GuildChannel>>(),
        Err(why) => {
            error!("Couldn't get channels: {why:?}");
            return Vec::new();
        }
    }
}
