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
    all::{ChannelId, Mentionable},
    builder::EditChannel,
};
use tracing::{error, info};

use crate::{
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_CHANNELS",
    required_bot_permissions = "MANAGE_CHANNELS | SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Apply a rate limit to a given channel.
pub async fn slowmode(
    ctx: Context<'_>,
    #[description = "The channel to slow down."]
    #[rename = "channel"]
    channel_id: Option<ChannelId>,
    #[description = "The amount of seconds to wait between each message."]
    #[min = 1]
    #[max = 21600]
    delay: Option<u16>,
) -> Result<(), Error> {
    let channel_id = match channel_id {
        Some(channel_id) => channel_id,
        None => {
            let reply =
                messages::error_reply(format!("You must specify a channel to slow down!"), true);
            ctx.send(reply).await?;

            return Ok(());
        }
    };
    let delay = match delay {
        Some(delay) => delay,
        None => 0,
    };

    let user_name = ctx.author().name.clone();

    let guild = models::guilds::guild(ctx)?;
    let (guild_name, guild_channels) = (guild.name, models::channels::channels(ctx).await?);

    for mut guild_channel in guild_channels {
        let guild_channel_id = guild_channel.id;
        if guild_channel_id != channel_id {
            continue;
        }

        let (guild_channel_name, guild_channel_mention) =
            (channel_id.name(ctx).await?, channel_id.mention());

        let channel_builder = EditChannel::default().rate_limit_per_user(delay);

        let result = match guild_channel.edit(ctx, channel_builder).await {
            Ok(_) => {
                info!(
                    "@{user_name} applied {delay}s limit to #{guild_channel_name} in {guild_name}"
                );

                if delay == 1 {
                    Ok(format!(
                        "Users now have to wait `{delay}` second between each message in {guild_channel_mention}."
                    ))
                } else if delay > 1 {
                    Ok(format!(
                        "Users now have to wait `{delay}` seconds between each message in {guild_channel_mention}."
                    ))
                } else {
                    Ok(format!("Removed rate limit from {guild_channel_mention}."))
                }
            }
            Err(why) => {
                error!("Failed to apply {delay}s limit to #{guild_channel_name} in {guild_name}: {why:?}");
                Err(format!(
                    "An error occurred whilst applying rate limit to {guild_channel_mention}."
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
