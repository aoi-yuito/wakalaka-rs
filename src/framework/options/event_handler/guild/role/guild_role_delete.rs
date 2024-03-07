// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, RoleId};
use tracing::info;

use crate::{utils::models, Error, SContext};

pub(crate) async fn handle(
    ctx: &SContext,
    guild_id: &GuildId,
    role_id: &RoleId,
) -> Result<(), Error> {
    let guild = models::guilds::guild_from_id_raw(ctx, guild_id)?;
    let guild_name = &guild.name;

    let role = models::roles::role_from_id_raw(ctx, guild_id, role_id).await?;
    let role_name = &role.name;

    info!("@{role_name} deleted from {guild_name}");

    Ok(())
}
