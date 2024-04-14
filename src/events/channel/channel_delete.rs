// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::GuildChannel;
use tracing::info;
use wakalaka_core::types::{SContext, Throwable};
use wakalaka_utils::accessors;

pub(crate) async fn handle_channel_delete_event(
    ctx: &SContext,
    guild_channel: &GuildChannel,
) -> Throwable<()> {
    let channel_name = &guild_channel.name;

    let guild_id = &guild_channel.guild_id;
    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    info!("#{channel_name:?} deleted in {guild_name:?}");

    Ok(())
}
