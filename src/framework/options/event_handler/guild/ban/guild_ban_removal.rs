// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, User};
use tracing::info;

use crate::{utils::models, Error, SContext};

pub(crate) async fn handle(ctx: &SContext, guild_id: &GuildId, user: &User) -> Result<(), Error> {
    let guild = models::guilds::guild_from_id_raw(ctx, &guild_id)?;
    let guild_name = guild.name;

    let user_name = &user.name;

    info!("@{user_name} unbanned from {guild_name}");

    Ok(())
}
