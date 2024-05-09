// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Role;

use wakalaka_core::{
    accessors,
    types::{SerenityContext, Throwable},
};

pub(crate) async fn handle_guild_role_create_event(ctx: &SerenityContext, role: &Role) -> Throwable<()> {
    let role_name = &role.name;

    let guild_id = &role.guild_id;
    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    tracing::info!("@{role_name} created in {guild_name}");

    Ok(())
}
