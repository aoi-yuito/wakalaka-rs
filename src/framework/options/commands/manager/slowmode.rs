// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::{
    all::{GuildChannel, Mentionable},
    builder::EditChannel,
};
use tracing::info;

use crate::{
    utils::{components, models},
    Context, Error,
};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_CHANNELS",
    required_bot_permissions = "MANAGE_CHANNELS | SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Apply a timed cooldown to a channel.
pub(super) async fn slowmode(
    ctx: Context<'_>,
    #[description = "The channel to slow down."] channel: Option<GuildChannel>,
    #[description = "The amount of seconds to wait between each message."]
    #[min = 1]
    #[max = 21600]
    delay: Option<u16>,
) -> Result<(), Error> {
    let channel = match channel {
        Some(channel) => channel,
        None => ctx.guild_channel().await.unwrap(),
    };
    let channel_id = channel.id;

    let delay = delay.unwrap_or(0);

    let author = ctx.author();
    let author_name = &author.name;

    let guild = models::guilds::guild(ctx)?;
    let guild_id = guild.id;
    let guild_name = &guild.name;

    let guild_channels = models::channels::channels(ctx, &guild_id).await?;
    for mut guild_channel in guild_channels {
        let guild_channel_id = guild_channel.id;
        if guild_channel_id != channel_id {
            continue;
        }

        let guild_channel_name = guild_channel.name.clone();
        let guild_channel_mention = guild_channel.mention();

        let guild_channel_builder = EditChannel::default().rate_limit_per_user(delay);

        let result = match guild_channel.edit(ctx, guild_channel_builder).await {
            Ok(_) => {
                info!(
                    "@{author_name} applied {delay}-second cooldown in #{guild_channel_name} in {guild_name}",
                );

                if delay == 1 {
                    Ok(format!(
                        "{guild_channel_mention} now has a cooldown of {delay} second per message."
                    ))
                } else if delay > 1 {
                    Ok(format!(
                        "{guild_channel_mention} now has a cooldown of {delay} seconds per message."
                    ))
                } else {
                    Ok(format!("{guild_channel_mention} is no longer on cooldown."))
                }
            }
            Err(why) => Err(format!(
                "An error occurred whilst slowing down {guild_channel_mention}: {why}",
            )),
        };

        let reply = match result {
            Ok(message) => components::replies::ok_reply_embed(message, true),
            Err(message) => components::replies::error_reply_embed(message, true),
        };

        ctx.send(reply).await?;
    }

    Ok(())
}
