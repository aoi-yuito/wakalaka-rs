// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::GuildId;

use wakalaka_core::types::{SerenityContext, Throwable};

pub(crate) async fn handle_cache_ready_event(
    ctx: &SerenityContext,
    guild_ids: &Vec<GuildId>,
) -> Throwable<()> {
    let guild_id_count = guild_ids.len();
    if guild_id_count == 1 {
        tracing::info!("Readied cache for {guild_id_count} server");
    } else {
        tracing::info!("Readied cache for {guild_id_count} servers");
    }

    for guild_id in guild_ids {
        let cmds = wakalaka_commands::gather_all_commands().await;

        let cmd_count = cmds.len();
        if cmd_count > 0 {
            poise::builtins::register_in_guild(ctx, &cmds, *guild_id).await?;
        }
    }

    Ok(())
}
