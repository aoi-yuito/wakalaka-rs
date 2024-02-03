// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

use std::sync::Arc;

use tokio::time::Duration;
use tracing::{error, info};

use crate::{
    check_restricted_guild,
    serenity::{ActivityData, Ready},
    utility::models,
    Data,
};

pub async fn handle(ready: &Ready, ctx: &crate::serenity::Context, data: &Data) {
    let pool = &data.pool;

    let guild_ids = ctx.cache.guilds(); // Cannot have this as a utility function as it would refuse to find the IDs in cache.
    for guild_id in &guild_ids {
        let guild_name = models::guilds::guild_name_from_guild_id_raw(ctx, *guild_id)
            .await
            .unwrap();

        let restricted_guild = check_restricted_guild!(&pool, &guild_id);
        if restricted_guild {
            if let Err(why) = guild_id.leave(ctx).await {
                error!("Couldn't leave {guild_name}: {why:?}");
            }

            return;
        }
    }

    let guild_count = guild_ids.len();

    let user_name = &ready.user.name;

    info!("Connected to {guild_count} guild(s) as @{user_name}");

    let ctx = Arc::new(ctx.clone());
    tokio::spawn(async move {
        loop {
            set_activity(&ctx).await;

            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
}

async fn set_activity(ctx: &crate::serenity::Context) {
    let guild_count = ctx.cache.guilds().len();

    let ytpmv = "Blue As You Are";

    let activity = format!("{ytpmv:?} in {guild_count} guild(s)");
    ctx.set_activity(Some(ActivityData::listening(&activity)));
}
