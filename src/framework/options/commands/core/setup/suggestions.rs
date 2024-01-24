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

use serenity::all::ChannelId;
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
    owners_only,
    guild_only,
    ephemeral
)]
/// Set up a channel for suggestions to be sent to.
pub async fn suggestions(
    ctx: Context<'_>,
    #[description = "The channel to send suggestions to."]
    #[rename = "channel"]
    channel_id: ChannelId,
) -> Result<(), Error> {
    let data = ctx.data();
    let pool = &data.pool;

    let guild_id = models::guilds::guild_id(ctx).await;

    let guild_channels = models::channels::channels(ctx).await;
    for guild_channel in guild_channels {
        let (guild_channel_id, guild_channel_name) = (guild_channel.id, &guild_channel.name());

        if guild_channel_id != channel_id {
            continue;
        }

        let query =
            guilds::update_guilds_set_suggestions_channel_id(guild_channel_id, guild_id, pool)
                .await;
        if let Err(why) = query {
            error!("Couldn't configure #{guild_channel_name} for suggestions: {why:?}");

            let reply = messages::error_reply(
                format!("Sorry, but I couldn't set <#{guild_channel_id}> to be for suggestions."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Err(why.into());
        }

        info!("Configured #{guild_channel_name} for suggestions");

        let reply = messages::ok_reply(
            format!("I've set <#{guild_channel_id}> to be for suggestions."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }
    }

    Ok(())
}
