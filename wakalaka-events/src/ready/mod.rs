// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub(super) mod cache;
pub(super) mod shards;

use wakalaka_core::types::{SerenityContext, SerenityReady, Throwable};

pub(super) async fn handle_ready_event(ctx: &SerenityContext, ready: &SerenityReady) -> Throwable<()> {
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
