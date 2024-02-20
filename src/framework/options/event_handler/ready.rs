// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::info;

use crate::{Error, SContext, SReady};

pub(crate) async fn handle(ctx: &SContext, ready: &SReady) -> Result<(), Error> {
    let bot = &ready.user;
    let bot_name = &bot.name;

    let guild_ids = ctx.cache.guilds();

    let guild_id_count = guild_ids.len();
    if guild_id_count == 1 {
        info!("Connected to {guild_id_count} guild as @{bot_name}");
    } else {
        info!("Connected to {guild_id_count} guilds as @{bot_name}");
    }

    Ok(())
}
