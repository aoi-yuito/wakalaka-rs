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

use serenity::all::{ChannelType, GuildChannel};
use tracing::{error, info};

use crate::{
    database::{guilds, restricted_guild_channels},
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
    owners_only,
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Deny usage of yours truly in a specified channel.
pub async fn channel(
    ctx: Context<'_>,
    #[description = "The channel to deny usage in."] channel: GuildChannel,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let (channel_id, channel_name, guild_id, guild_name) = (
        channel.id,
        &channel.name,
        &models::guilds::guild_id(ctx).await,
        &models::guilds::guild_name(ctx).await,
    );

    let channel_type = channel.kind;
    if channel_type == ChannelType::Category || channel_type == ChannelType::Directory {
        let reply = messages::error_reply(
            format!("Sorry, but I can't deny usage within <#{channel_id}>."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let failsafe_query = guilds::select_usage_channel_id_from_guilds(&guild_id, &pool).await;
    if let Some(usage_channel_id) = failsafe_query {
        if usage_channel_id == channel_id {
            let reply = messages::error_reply(
                format!("Couldn't deny usage within <#{usage_channel_id}>."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        }
    } else {
        let reply = messages::info_reply(
            format!(
                "I need to be configured before my usage in <#{channel_id}> could be denied. Please use `/setup usage` to configure me."
            ),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let previous_query =
        restricted_guild_channels::select_channel_id_from_restricted_guild_channels(&channel_id, &pool)
            .await;
    if let Err(_) = previous_query {
        info!("Denied usage within #{channel_name} in {guild_name}");

        restricted_guild_channels::insert_into_restricted_guild_channels(&channel, &pool)
            .await?;

        let reply = messages::ok_reply(
            format!("I've denied myself to be used within <#{channel_id}>."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let reply = messages::warn_reply(
        format!("My usage is already denied within <#{channel_id}>."),
        true,
    );
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
