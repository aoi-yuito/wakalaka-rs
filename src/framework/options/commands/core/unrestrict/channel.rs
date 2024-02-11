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

use serenity::all::{ChannelType, GuildChannel, Mentionable};
use tracing::info;

use crate::{
    database::{guilds, restricted_guild_channels},
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Allow usage of yours truly in a specified channel.
pub async fn channel(
    ctx: Context<'_>,
    #[description = "The channel to allow usage in."] channel: GuildChannel,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let (channel_id, channel_name, channel_mention, guild_id) = (
        channel.id,
        &channel.name,
        channel.mention(),
        &models::guilds::guild_id(ctx)?,
    );
    let guild_name = models::guilds::guild_name(ctx, *guild_id);

    let (user_id, owner_id) = (ctx.author().id, models::guilds::owner_id(ctx)?);
    if user_id != owner_id {
        let reply = messages::info_reply(None, 
            format!("Only 👑 can allow usage within {channel_mention}."),
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let channel_type = channel.kind;
    if channel_type == ChannelType::Category || channel_type == ChannelType::Directory {
        let reply = messages::error_reply(None, 
            format!("Cannot allow usage within {channel_mention}!"),
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let failsafe_query = guilds::select_usage_channel_id_from_guilds(&guild_id, &pool).await;
    let result = match failsafe_query {
        Some(usage_channel_id) if usage_channel_id == channel_id => {
            let usage_channel_mention = usage_channel_id.mention();
            Err(format!("Usage within {usage_channel_mention} is already allowed."))
        }
        None => {
            Err(format!(
                "Yours truly must be configured before usage in {channel_mention} could be allowed. Please use `/setup usage` to configure yours truly."
            ))
        }
        _ => {
            let previous_query = restricted_guild_channels::select_channel_id_from_restricted_guild_channels(&channel_id, &pool).await;
            match previous_query {
                Ok(_) => {
                    info!("Allowed usage within #{channel_name} in {guild_name}");
                    restricted_guild_channels::delete_from_restricted_guild_channels(&channel, &pool).await?;
                    Ok(format!("Allowed usage of yours truly within {channel_mention}."))
                }
                _ => Err(format!("Usage within {channel_mention} is already allowed!"))
            }
        }
    };

    let reply = match result {
        Ok(message) => messages::ok_reply(None, message, true),
        Err(message) => messages::error_reply(None, message, true),
    };
    ctx.send(reply).await?;

    Ok(())
}
