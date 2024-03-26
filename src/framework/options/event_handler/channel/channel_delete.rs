// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::GuildChannel;
use tracing::info;

use crate::{utils::models, SContext, Throwable};

pub(crate) async fn handle(ctx: &SContext, channel: &GuildChannel) -> Throwable<()> {
    let guild_id = channel.guild_id;
    let guild = models::guilds::guild_from_id_raw(ctx, &guild_id)?;
    let guild_name = guild.name;

    let channel_name = &channel.name;

    info!("#{channel_name} deleted from {guild_name}");

    Ok(())
}
