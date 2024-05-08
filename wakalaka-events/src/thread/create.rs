// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ChannelType, GuildChannel};

use wakalaka_core::{
    accessors,
    types::{SerenityContext, Throwable},
};

pub(crate) async fn handle_thread_create_event(
    ctx: &SerenityContext,
    guild_channel: &GuildChannel,
) -> Throwable<()> {
    let channel_type = guild_channel.kind;
    if channel_type != ChannelType::PublicThread || channel_type != ChannelType::PrivateThread {
        return Ok(());
    }

    let thread_name = &guild_channel.name;

    let guild_id = &guild_channel.guild_id;
    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    tracing::info!("#{thread_name} created in {guild_name}");

    Ok(())
}
