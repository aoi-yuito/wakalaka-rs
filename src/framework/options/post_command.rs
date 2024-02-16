// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::info;

use crate::{utils::models, Context};

pub(crate) async fn handle(ctx: Context<'_>) {
    let user_name = &ctx.author().name;

    let command_name = &ctx.command().qualified_name;

    let channel_id = ctx.channel_id();
    let channel_name = models::channels::name(ctx, &channel_id).await;

    let guild = models::guilds::guild(ctx).unwrap();
    let guild_name = &guild.name;

    info!("@{user_name} invoked {command_name:?} in #{channel_name} in {guild_name}");
}
