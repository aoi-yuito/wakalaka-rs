// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ChannelType, EditChannel, GuildChannel, Mentionable};

use wakalaka_core::{
    accessors, builders,
    types::{Context, Throwable},
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
/// Apply slowmode to a channel.
pub(super) async fn slowmode(
    ctx: Context<'_>,
    #[description = "Channel to slow down."]
    #[rename = "channel"]
    guild_channel: Option<GuildChannel>,
    #[description = "Seconds a user must wait per message, if any."]
    #[min = 1]
    #[max = 21600]
    seconds: Option<u16>,
) -> Throwable<()> {
    let seconds = seconds.unwrap_or(0); // Without providance of seconds, slowmode is no more.

    let author = ctx.author();
    let author_name = &author.name;

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = &guild.name;

    let ctx_channel_id = ctx.channel_id();
    let ctx_channel = ctx_channel_id.to_channel(ctx).await?;

    let mut guild_channel = guild_channel.unwrap_or(
        ctx_channel
            .guild()
            .ok_or("Failed to convert channel to guild channel")?,
    );
    let guild_channel_name = guild_channel.name.clone();
    let guild_channel_mention = guild_channel.mention();
    let guild_channel_kind = guild_channel.kind;

    let editable_channel_types = vec![
        ChannelType::Text,
        ChannelType::Voice,
        ChannelType::Stage,
        ChannelType::Forum,
    ];
    if !editable_channel_types.contains(&guild_channel_kind) {
        let reply = builders::replies::build_error_reply_with_embed(
            format!("Cannot edit #{guild_channel_mention}."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let editable_guild_channel = EditChannel::default().rate_limit_per_user(seconds);

    let before_reply = if seconds < 1 {
        builders::replies::build_reply_with_embed(
            format!("Removing slowmode from {guild_channel_mention}..."),
            true,
        )
    } else {
        builders::replies::build_reply_with_embed(
            format!("Applying slowmode to {guild_channel_mention}..."),
            true,
        )
    };

    let result = match guild_channel.edit(ctx, editable_guild_channel).await {
        Ok(_) => {
            tracing::info!("@{author_name} slowed #{guild_channel_name} down in {guild_name}");

            if seconds < 1 {
                Ok(format!("{guild_channel_mention} is no longer slowed down."))
            } else {
                Ok(format!("{guild_channel_mention} has been slowed down."))
            }
        }
        Err(e) => {
            tracing::error!(
                "@{author_name} failed to slow #{guild_channel_name} down in {guild_name}: {e:?}"
            );

            Err(format!(
                "An error occurred while trying to slow {guild_channel_mention} down."
            ))
        }
    };

    let reply_handle = ctx.send(before_reply).await?;

    let after_reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(emsg) => builders::replies::build_error_reply_with_embed(emsg, true),
    };

    reply_handle.edit(ctx, after_reply).await?;

    Ok(())
}
