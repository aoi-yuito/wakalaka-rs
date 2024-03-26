// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{ChannelType, GuildChannel};
use tracing::info;

use crate::{utils::models, SContext, Throwable};

pub(crate) async fn handle(ctx: &SContext, category: &GuildChannel) -> Throwable<()> {
    if category.kind != ChannelType::Category {
        return Ok(());
    }

    let guild_id = category.guild_id;
    let guild = models::guilds::guild_from_id_raw(ctx, &guild_id)?;
    let guild_name = guild.name;

    let category_name = &category.name;

    info!(">{category_name} deleted from {guild_name}");

    Ok(())
}
