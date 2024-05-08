// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ChannelType, GuildChannel};

use wakalaka_core::{
    accessors,
    types::{SerenityContext, Throwable},
};

pub(crate) async fn handle_thread_delete_event(
    ctx: &SerenityContext,
    guild_channel: &Option<GuildChannel>,
) -> Throwable<()> {
    if let Some(guild_channel) = guild_channel {
        let channel_type = guild_channel.kind;
        if channel_type != ChannelType::PublicThread || channel_type != ChannelType::PrivateThread {
            return Ok(());
        }

        let thread_id = &guild_channel.name;

        let guild_id = &guild_channel.guild_id;
        let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
        let guild_name = guild.name;

        tracing::info!("#{thread_id} deleted in {guild_name}");
    }

    Ok(())
}
