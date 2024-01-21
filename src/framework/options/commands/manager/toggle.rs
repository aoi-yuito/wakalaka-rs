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
    utility::{self, components::messages},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("slowmode"),
    category = "Manager",
    required_permissions = "MANAGE_CHANNELS",
    guild_only,
    subcommand_required,
    ephemeral
)]
pub(crate) async fn toggle(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_CHANNELS",
    guild_only,
    ephemeral
)]
/// Reduce the rate of messages in a channel.
pub(crate) async fn slowmode(
    ctx: Context<'_>,
    #[description = "The channel to toggle slowmode in, if any."]
    #[rename = "channel"]
    channel_id: Option<ChannelId>,
    #[description = "The delay between messages. (0-60s)"]
    #[min = 0]
    #[max = 60]
    delay: Option<u16>,
) -> Result<(), Error> {
    let channel_id = utility::channels::channel_id(ctx, channel_id).await;
    let delay = match delay {
        Some(delay) => delay,
        None => 0,
    };

    let user_name = &ctx.author().name;

    let guild = utility::guilds::guild(ctx).await;
    let (guild_name, guild_channels) = (guild.name, utility::guilds::channels(ctx).await);
    for mut guild_channel in guild_channels {
        let guild_channel_id = guild_channel.id;
        if channel_id != guild_channel_id {
            continue;
        }

        let guild_channel_name = guild_channel.name.clone();

        let edit_channel = EditChannel::default().rate_limit_per_user(delay);

        if let Err(why) = guild_channel.edit(&ctx, edit_channel).await {
            error!(
                "Couldn't slow #{guild_channel_name} down for {delay}s in {guild_name}: {why:?}"
            );

            let reply =
                messages::error_reply(format!("Couldn't slow <#{guild_channel_id}> down."), true);
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(Error::from(why));
            }

            return Err(Error::from(why));
        } else {
            info!("@{user_name} slowed #{guild_channel_name} down to {delay}s in {guild_name}");

            let reply = messages::ok_reply(
                format!("Slowed <#{guild_channel_id}> down to `{delay}`s."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(Error::from(why));
            }
        }
    }

    Ok(())
}