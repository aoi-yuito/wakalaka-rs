// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::InviteDeleteEvent;

use wakalaka_core::{
    accessors,
    types::{SContext, Throwable},
};

pub(crate) async fn handle_invite_delete_event(
    ctx: &SContext,
    delete_evt: &InviteDeleteEvent,
) -> Throwable<()> {
    let guild_id = &match delete_evt.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return Ok(());
        }
    };
    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    let code = &delete_evt.code;

    tracing::info!("Deleted {code} from {guild_name}");

    Ok(())
}
