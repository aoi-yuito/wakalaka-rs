// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::GuildId;
use tracing::info;
use wakalaka_core::types::{SContext, Throwable};

use crate::commands;

pub(crate) async fn handle_cache_ready_event(
    ctx: &SContext,
    guild_ids: &Vec<GuildId>,
) -> Throwable<()> {
    let guild_id_count = guild_ids.len();
    if guild_id_count == 1 {
        info!("Readied cache for {guild_id_count} server");
    } else {
        info!("Readied cache for {guild_id_count} servers");
    }

    for guild_id in guild_ids {
        commands::register_guild_commands(ctx, guild_id).await?; // Globally registered commands are fucking hassle because they bitch and take an hour to update.
    }

    Ok(())
}
