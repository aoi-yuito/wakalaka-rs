// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub(super) mod cache_ready;
pub(super) mod shards_ready;

use tracing::info;

use crate::{SContext, SReady, Throwable};

pub(crate) async fn handle(ctx: &SContext, ready: &SReady) -> Throwable<()> {
    let bot = &ready.user;
    let bot_name = &bot.name;

    let guild_ids = ctx.cache.guilds();

    let guild_id_count = guild_ids.len();
    if guild_id_count == 1 {
        info!("@{bot_name} connected to {guild_id_count} server");
    } else {
        info!("@{bot_name} connected to {guild_id_count} servers");
    }

    Ok(())
}
