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

use serenity::all::{Channel, Mentionable};
use tracing::{error, info};

use crate::{
    database::guilds,
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
/// Set up a channel for suggestions to be sent to.
pub async fn suggestions(
    ctx: Context<'_>,
    #[description = "The channel to send suggestions to."] channel: Channel,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let (channel_id, channel_mention) = (channel.id(), channel.mention());

    let (user_id, owner_id) = (ctx.author().id, models::guilds::owner_id(ctx)?);
    if user_id != owner_id {
        let reply =
            messages::info_reply(format!("Only 👑 can configure {channel_mention}."), true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let guild_id = models::guilds::guild_id(ctx)?;

    let guild_channels = models::channels::channels(ctx).await?;
    for guild_channel in guild_channels {
        let (guild_channel_id, guild_channel_name, guild_channel_mention) = (
            guild_channel.id,
            &guild_channel.name(),
            guild_channel.mention(),
        );
        if guild_channel_id != channel_id {
            continue;
        }

        let result = match guilds::update_guilds_set_suggestions_channel_id(
            guild_channel_id,
            guild_id,
            pool,
        )
        .await
        {
            Ok(_) => {
                info!("Configured #{guild_channel_name} for suggestions");
                Ok(format!(
                    "Configured {guild_channel_mention} to be for suggestions."
                ))
            }
            Err(why) => {
                error!("Failed to configure #{guild_channel_name} for suggestions: {why:?}");
                Err(format!(
                    "An error occurred whilst configuring {guild_channel_mention} to be for suggestions."
                ))
            }
        };

        let reply = match result {
            Ok(message) => messages::ok_reply(message, true),
            Err(message) => messages::error_reply(message, true),
        };
        ctx.send(reply).await?;
    }

    Ok(())
}
