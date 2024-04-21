// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ChannelType, EditChannel, GuildChannel, Mentionable};
use tracing::{error, info};
use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::{accessors, builders};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_CHANNELS",
    required_bot_permissions = "MANAGE_CHANNELS | SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Apply slowmode to a channel.
pub(super) async fn slowmode(
    ctx: Context<'_>,
    #[description = "Channel to slow down, if any."]
    #[rename = "channel"]
    guild_channel: Option<GuildChannel>,
    #[description = "Seconds to wait between messages, if any."]
    #[min = 1]
    #[max = 21600]
    seconds: Option<u16>,
) -> Throwable<()> {
    let seconds = seconds.unwrap_or(0); // Back to being chronically online if none specified.

    let author = ctx.author();
    let author_name = &author.name;

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_id = &guild.id;
    let guild_name = &guild.name;

    let ctx_guild_channels = accessors::channels::gather_all_guild_channels(ctx, guild_id).await?;
    for mut ctx_guild_channel in ctx_guild_channels {
        let ctx_guild_channel_type = ctx_guild_channel.kind;
        if ctx_guild_channel_type != ChannelType::Text
            || ctx_guild_channel_type != ChannelType::PublicThread
            || ctx_guild_channel_type != ChannelType::PrivateThread
        {
            continue;
        }

        let ctx_guild_channel_id = ctx_guild_channel.id;

        let guild_channel = guild_channel.as_ref().unwrap_or(&ctx_guild_channel); // We do a little slowmode on EVERY. SINGLE. CHANNEL. if none specified.
        let guild_channel_id = guild_channel.id;
        if guild_channel_id != ctx_guild_channel_id {
            // Clueless statement. Touch it and I'll tittyfuck your jumper.
            continue;
        }

        let guild_channel_name = guild_channel.name.clone();
        let guild_channel_mention = guild_channel.mention();

        let edited_guild_channel = EditChannel::default().rate_limit_per_user(seconds);

        let result = match ctx_guild_channel.edit(ctx, edited_guild_channel).await {
            Ok(_) => {
                info!("@{author_name} slowed #{guild_channel_name} down in {guild_name}");

                if seconds < 1 {
                    Ok(format!("{guild_channel_mention} is no longer slowed down."))
                } else {
                    Ok(format!("{guild_channel_mention} has been slowed down."))
                }
            }
            Err(e) => {
                error!("@{author_name} failed to slow #{guild_channel_name} down in {guild_name}: {e:?}");

                Err(format!(
                    "An error occurred while trying to slow {guild_channel_mention} down."
                ))
            }
        };

        let reply = match result {
            Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
            Err(msg) => builders::replies::build_error_reply_with_embed(msg, true),
        };

        ctx.send(reply).await?;
    }

    Ok(())
}
