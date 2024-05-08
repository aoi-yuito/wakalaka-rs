// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::InviteCreateEvent;

use wakalaka_core::{
    accessors,
    types::{SerenityContext, Throwable},
};

pub(crate) async fn handle_invite_create_event(
    ctx: &SerenityContext,
    create_evt: &InviteCreateEvent,
) -> Throwable<()> {
    let guild_id = &match create_evt.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return Ok(());
        }
    };
    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    let code = &create_evt.code;

    if let Some(inviter) = &create_evt.inviter {
        let inviter_name = &inviter.name;

        tracing::info!("@{inviter_name} created {code} to {guild_name}");
    } else {
        tracing::info!("Created {code} to {guild_name}");
    }

    Ok(())
}
