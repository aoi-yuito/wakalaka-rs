// Copyright (c) 2024 Kawaxte
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::Arc;

use serenity::gateway::ActivityData;
use tokio::time::Duration;

use tracing::info;

use crate::{Error, SContext, SReady};

pub(crate) async fn handle(ctx: &SContext, ready: &SReady) -> Result<(), Error> {
    let user_name = &ready.user.name;

    let guild_ids = ctx.cache.guilds();

    let guild_id_count = guild_ids.len();
    if guild_id_count == 1 {
        info!("Connected to {guild_id_count} guild as @{user_name}");
    } else {
        info!("Connected to {guild_id_count} guilds as @{user_name}");
    }

    let ctx = Arc::new(ctx.clone());
    tokio::spawn(async move {
        loop {
            set_activity(&ctx, &guild_id_count).await;

            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });

    Ok(())
}

async fn set_activity(ctx: &crate::serenity::Context, count: &usize) {
    let ytpmv = "Blue As You Are";

    let activity = format!("{ytpmv:?} in {count} guild(s)");
    ctx.set_activity(Some(ActivityData::listening(&activity)));
}
