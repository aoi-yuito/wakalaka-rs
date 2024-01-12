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

use tracing::info;

use crate::{Context, Error};

/// Restarts yours truly.
#[poise::command(slash_command, owners_only)]
pub(crate) async fn restart(ctx: Context<'_>) -> Result<(), Error> {
    let message = "Restarting yours truly...";
    let _ = ctx.reply(message).await;

    let shard_manager = ctx.framework().shard_manager.clone();
    let shard_ids = shard_manager
        .runners
        .lock()
        .await
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    for shard_id in shard_ids {
        info!("Restarting shard {}", shard_id);
        shard_manager.restart(shard_id).await;
    }

    Ok(())
}
