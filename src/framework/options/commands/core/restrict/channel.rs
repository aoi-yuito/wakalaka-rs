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
    database::guild_channels,
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
    owners_only,
    guild_only,
    ephemeral
)]
/// Deny usage of yours truly in a specified channel.
pub async fn channel(
    ctx: Context<'_>,
    #[description = "The channel to deny usage in."] channel: GuildChannel,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let current_channel_id = ctx.channel_id();

    let channel_type = channel.kind;
    if channel_type == ChannelType::Category {
        let reply = messages::warn_reply(
            format!("I'm afraid you can't deny usage of yours truly within a category."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let (channel_id, channel_name, guild_id, guild_name) = (
        channel.id,
        &channel.name,
        channel.guild_id,
        &models::guilds::guild_name(ctx).await,
    );
    if channel_id == current_channel_id {
        let reply = messages::warn_reply(
            format!("I'm afraid you can't deny usage of yours truly within <#{channel_id}>."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    // We don't want to try to deny access to a channel that's already been denied.
    let previous_query =
        guild_channels::select_restrict_from_guild_channels(&channel_id, &guild_id, pool).await;
    if let Ok(restricted) = previous_query {
        if !restricted {
            let reply = messages::warn_reply(
                format!("Usage of yours truly is already denied within <#{channel_id}>."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        }
    }

    let reply = messages::reply(
        format!("Denying usage of yours truly within <#{channel_id}>..."),
        true,
    );
    let handle = ctx.send(reply).await;

    let query =
        guild_channels::update_guild_channels_set_restrict(&channel_id, &guild_id, true, pool)
            .await;
    if let Ok(_) = query {
        info!("Denied usage within #{channel_name} in {guild_name}");

        if let Ok(message) = handle {
            let reply = messages::reply(
                format!("I've denied usage of yours truly within <#{channel_id}>."),
                true,
            );
            if let Err(why) = message.edit(ctx, reply).await {
                error!("Couldn't edit message: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        }
    } else {
        error!("Couldn't deny usage within #{channel_name} in {guild_name}");

        if let Ok(message) = handle {
            let reply = messages::error_reply(
                format!("Sorry, but I couldn't deny usage of yours truly within <#{channel_id}>."),
                true,
            );
            if let Err(why) = message.edit(ctx, reply).await {
                error!("Couldn't edit message: {why:?}");
                return Err(why.into());
            }
        }
    }

    Ok(())
}
