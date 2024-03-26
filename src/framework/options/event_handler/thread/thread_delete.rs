// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ChannelType, GuildChannel};
use tracing::info;

use crate::{utils::models, SContext, Throwable};

pub(crate) async fn handle(ctx: &SContext, full_thread: &Option<GuildChannel>) -> Throwable<()> {
    if let Some(thread) = full_thread {
        if thread.kind != ChannelType::PublicThread || thread.kind != ChannelType::PrivateThread {
            return Ok(());
        }

        let guild_id = thread.guild_id;
        let guild = models::guilds::guild_from_id_raw(ctx, &guild_id)?;
        let guild_name = guild.name;

        let channel_name = &thread.name;

        info!("#{channel_name} created in {guild_name}");
    }

    Ok(())
}
