// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::GuildId;
use tracing::info;
use wakalaka_core::types::{SContext, Throwable};

pub(crate) async fn handle_cache_ready_event(
    _ctx: &SContext,
    guild_ids: &Vec<GuildId>,
) -> Throwable<()> {
    let guild_id_count = guild_ids.len();
    if guild_id_count == 1 {
        info!("Readied cache for {guild_id_count} server");
    } else {
        info!("Readied cache for {guild_id_count} servers");
    }

    // TODO: register guild cmds here

    Ok(())
}
