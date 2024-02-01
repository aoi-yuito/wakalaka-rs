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

use serenity::{all::ChannelId, builder::EditChannel};
use tracing::{error, info};

use crate::{
    check_restricted_guild_channel,
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_CHANNELS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Reduce the rate of messages in a channel.
pub async fn slowmode(
    ctx: Context<'_>,
    #[description = "The channel to slow down, if any."]
    #[rename = "channel"]
    channel_id: Option<ChannelId>,
    #[description = "Time between messages. (seconds)"]
    #[min = 0]
    #[max = 60]
    delay: Option<u16>,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    if delay.is_some() {
        if let Some(delay) = delay {
            if delay > 60 {
                let reply =
                    messages::info_reply(format!("Delay must be up to `60` seconds."), true);
                ctx.send(reply).await?;

                return Ok(());
            }
        }
    }

    let channel_id = match channel_id {
        Some(channel_id) => channel_id,
        None => {
            let reply =
                messages::info_reply(format!("You must specify a channel to slow down."), true);
            ctx.send(reply).await?;

            return Ok(());
        }
    };
    let delay = match delay {
        Some(delay) => delay,
        None => 0,
    };

    let user_name = models::author_name(ctx)?;

    let guild = models::guilds::guild(ctx)?;
    let (guild_name, guild_channels) = (guild.name, models::channels::channels(ctx).await?);

    for mut guild_channel in guild_channels {
        let guild_channel_id = guild_channel.id;
        if channel_id != guild_channel_id {
            continue;
        }

        let guild_channel_name = guild_channel.name.clone();

        let channel_builder = EditChannel::default().rate_limit_per_user(delay);

        let result = match guild_channel.edit(ctx, channel_builder).await {
            Ok(_) => {
                info!("@{user_name} slowed #{guild_channel_name} down to {delay} second(s) in {guild_name}");
                Ok(format!(
                    "I've slowed <#{guild_channel_id}> down to `{delay}` second(s)."
                ))
            }
            Err(why) => {
                error!("Couldn't slow #{guild_channel_name} down for {delay} second(s) in {guild_name}: {why:?}");
                Err(format!(
                    "Sorry, but I couldn't slow <#{guild_channel_id}> down."
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
