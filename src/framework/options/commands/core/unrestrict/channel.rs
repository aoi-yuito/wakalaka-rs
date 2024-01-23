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
    database::restricted_guild_channels,
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
/// Allow usage of yours truly in a specified channel.
pub async fn channel(
    ctx: Context<'_>,
    #[description = "The channel to allow usage in."] channel: GuildChannel,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let channel_type = channel.kind;
    if channel_type == ChannelType::Category {
        let reply = messages::warn_reply(
            format!("I'm afraid you can't allow usage of yours truly within {channel_type:?} channel(s)."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let (channel_id, channel_name, guild_name) = (
        channel.id,
        &channel.name,
        &models::guilds::guild_name(ctx).await,
    );

    let previous_query =
        restricted_guild_channels::select_from_restricted_guild_channels_by_one(&channel_id, &pool)
            .await;
    if let Ok(_) = previous_query {
        info!("Allowed usage within #{channel_name} in {guild_name}");

        restricted_guild_channels::delete_from_restricted_guild_channels_by_one(&channel, &pool)
            .await?;

        let reply = messages::ok_reply(
            format!("I've allowed usage of yours truly within <#{channel_id}>."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let reply = messages::warn_reply(
        format!("Usage of yours truly is already allowed within <#{channel_id}>."),
        true,
    );
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
