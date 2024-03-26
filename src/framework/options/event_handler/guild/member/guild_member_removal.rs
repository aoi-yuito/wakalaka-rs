// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{GuildId, User};
use tracing::info;

use crate::{utils::models, SContext, Throwable};

pub(crate) async fn handle(ctx: &SContext, guild_id: &GuildId, user: &User) -> Throwable<()> {
    if user.bot || user.system {
        return Ok(());
    }

    let user_name = &user.name;

    let guild_name = models::guilds::name_raw(ctx, guild_id);

    info!("@{user_name} left {guild_name}");

    Ok(())
}
