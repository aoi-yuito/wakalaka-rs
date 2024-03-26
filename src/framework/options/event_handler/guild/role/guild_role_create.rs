// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Role;
use tracing::info;

use crate::{utils::models, SContext, Throwable};

pub(crate) async fn handle(ctx: &SContext, role: &Role) -> Throwable<()> {
    let guild_id = role.guild_id;
    let guild = models::guilds::guild_from_id_raw(ctx, &guild_id)?;
    let guild_name = &guild.name;

    let role_name = &role.name;

    info!("@{role_name} created in {guild_name}");

    Ok(())
}
