// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::GuildId;
use tracing::info;

use crate::{framework::options, SContext, Throwable};

pub(crate) async fn handle(ctx: &SContext, guild_ids: &Vec<GuildId>) -> Throwable<()> {
    let guild_id_count = guild_ids.len();
    if guild_id_count == 1 {
        info!("Readied cache for 1 server");
    } else {
        info!("Readied cache for {guild_id_count} servers");
    }

    for guild_id in guild_ids {
        options::commands::register_guild_commands(ctx, guild_id).await?;
    }

    Ok(())
}
