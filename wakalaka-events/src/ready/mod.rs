// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub(super) mod cache_ready;
pub(super) mod shards_ready;

use wakalaka_core::types::{SContext, SReady, Throwable};

pub(super) async fn handle_ready_event(ctx: &SContext, ready: &SReady) -> Throwable<()> {
    let bot = &ready.user;
    let bot_name = &bot.name;

    let guild_ids = ctx.cache.guilds();

    let guild_id_count = guild_ids.len();
    if guild_id_count == 1 {
        tracing::info!("Readied @{bot_name} in {guild_id_count} server");
    } else {
        tracing::info!("Readied @{bot_name} in {guild_id_count} servers");
    }

    Ok(())
}
