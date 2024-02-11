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
use tracing::info;

use crate::{
    serenity::{ActivityData, Ready},
    Error,
};

pub async fn handle(ready: &Ready, ctx: &crate::serenity::Context) -> Result<(), Error> {
    let guild_ids = ctx.cache.guilds();

    let guild_count = guild_ids.len();

    let user_name = &ready.user.name;

    if guild_count == 1 {
        info!("Connected to {guild_count} guild as @{user_name}");
    } else {
        info!("Connected to {guild_count} guilds as @{user_name}");
    }

    let ctx = Arc::new(ctx.clone());
    tokio::spawn(async move {
        loop {
            set_activity(&ctx, &guild_count).await;

            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });

    Ok(())
}

async fn set_activity(ctx: &crate::serenity::Context, guild_count: &usize) {
    let ytpmv = "Blue As You Are";

    let activity = format!("{ytpmv:?} in {guild_count} guild(s)");
    ctx.set_activity(Some(ActivityData::listening(&activity)));
}
