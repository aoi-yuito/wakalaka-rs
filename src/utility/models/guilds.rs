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

use serenity::all::{Guild, GuildChannel, GuildId, Member, UserId};
use tracing::{error, warn};

use crate::Context;

pub(crate) async fn channels_raw(
    ctx: &crate::serenity::Context,
    guild_id: GuildId,
) -> Vec<GuildChannel> {
    match guild_raw(ctx, guild_id).channels(&ctx).await {
        Ok(channels) => channels.values().cloned().collect::<Vec<GuildChannel>>(),
        Err(why) => {
            error!("Couldn't get channels: {why:?}");
            return Vec::new();
        }
    }
}

pub(crate) async fn channels(ctx: Context<'_>) -> Vec<GuildChannel> {
    match guild(ctx).await.channels(&ctx).await {
        Ok(channels) => channels.values().cloned().collect::<Vec<GuildChannel>>(),
        Err(why) => {
            error!("Couldn't get channels: {why:?}");
            return Vec::new();
        }
    }
}

pub(crate) async fn member(ctx: Context<'_>, guild_id: GuildId, user_id: UserId) -> Member {
    match guild_id.member(&ctx, user_id).await {
        Ok(member) => member,
        Err(why) => {
            error!("Couldn't get member: {why:?}");
            return Member::default();
        }
    }
}

pub(crate) async fn members_raw(ctx: &crate::serenity::Context, guild_id: &GuildId) -> Vec<Member> {
    match guild_id.members(&ctx, None, None).await {
        Ok(users) => users,
        Err(why) => {
            error!("Couldn't get members: {why:?}");
            return Vec::new();
        }
    }
}

pub(crate) async fn members(ctx: Context<'_>, guild_id: GuildId) -> Vec<Member> {
    match guild_id.members(&ctx, None, None).await {
        Ok(users) => users,
        Err(why) => {
            error!("Couldn't get members: {why:?}");
            return Vec::new();
        }
    }
}

pub(crate) async fn owner_id(ctx: Context<'_>) -> UserId {
    guild(ctx).await.owner_id
}

pub(crate) async fn guild_name_raw(ctx: &crate::serenity::Context, guild_id: GuildId) -> String {
    guild_raw(ctx, guild_id).name
}

pub(crate) async fn guild_name(ctx: Context<'_>) -> String {
    guild(ctx).await.name
}

pub(crate) async fn guild_id_raw(ctx: &crate::serenity::Context) -> GuildId {
    super::current_application_info_raw(ctx)
        .await
        .expect("Couldn't get current application info")
        .guild_id
        .expect("Couldn't find guild ID in current application")
}

pub(crate) async fn guild_id(ctx: Context<'_>) -> GuildId {
    guild(ctx).await.id
}

pub(crate) fn guild_raw(ctx: &crate::serenity::Context, guild_id: GuildId) -> Guild {
    let guild = {
        match ctx.cache.guild(guild_id) {
            Some(value) => value,
            None => {
                warn!("Couldn't get guild, setting default...");
                return Guild::default();
            }
        }
    };
    guild.clone()
}

pub(crate) async fn guild(ctx: Context<'_>) -> Guild {
    match ctx.guild() {
        Some(value) => value.clone(),
        None => {
            warn!("Couldn't get guild, setting default...");
            return Guild::default();
        }
    }
}
