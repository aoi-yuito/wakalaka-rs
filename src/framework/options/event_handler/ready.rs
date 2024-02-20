// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::Arc;

use serenity::gateway::ActivityData;
use tokio::time::{interval, Duration, Interval};

use tracing::info;

use crate::{Error, SContext, SReady};

pub(crate) async fn handle(ctx: &SContext, ready: &SReady) -> Result<(), Error> {
    let bot = &ready.user;
    let bot_name = &bot.name;

    let guild_ids = ctx.cache.guilds();

    let guild_id_count = guild_ids.len();
    if guild_id_count == 1 {
        info!("Connected to {guild_id_count} guild as @{bot_name}");
    } else {
        info!("Connected to {guild_id_count} guilds as @{bot_name}");
    }

    let ctx = Arc::new(ctx.clone());

    let interval = interval(Duration::from_secs(30));

    set_activity(&ctx, interval, &guild_id_count).await;

    Ok(())
}

async fn set_activity(ctx: &SContext, mut interval: Interval, count: &usize) {
    let ytpmv = "Blue As You Are";

    let activity = format!("{ytpmv:?} in {count} guild(s)");

    loop {
        ctx.set_activity(Some(ActivityData::listening(&activity)));

        interval.tick().await;
    }
}
