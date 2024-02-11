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

use serenity::all::ShardId;

use crate::{utility::components::messages, Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    owners_only,
    user_cooldown = 5,
    ephemeral
)]
/// Restart yours truly to her former glory.
pub async fn restart(ctx: Context<'_>) -> Result<(), Error> {
    let reply = messages::reply("Restarting...", true);
    ctx.send(reply).await?;

    let manager = Arc::new(ctx.framework().shard_manager);

    let shard_ids = manager
        .runners
        .lock()
        .await
        .keys()
        .cloned()
        .collect::<Vec<ShardId>>();
    for shard_id in shard_ids {
        manager.restart(shard_id).await;
    }

    Ok(())
}
