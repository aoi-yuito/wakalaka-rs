// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ChannelType, GuildChannel};

use wakalaka_core::{
    accessors,
    types::{SContext, Throwable},
};

pub(crate) async fn handle_category_create_event(
    ctx: &SContext,
    guild_channel: &GuildChannel,
) -> Throwable<()> {
    let channel_type = guild_channel.kind;
    if channel_type != ChannelType::Category {
        return Ok(());
    }

    let category_name = &guild_channel.name;

    let guild_id = &guild_channel.guild_id;
    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    tracing::info!("{category_name} created in {guild_name}");

    Ok(())
}
