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

use tokio::time::Duration;
use tracing::info;

use crate::{Context, Error};

/// Puts yours truly to sleep.
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub(crate) async fn shutdown(
    ctx: Context<'_>,
    #[description = "Seconds before yours truly falls asleep."] delay: u64,
) -> Result<(), Error> {
    if delay < 1 || delay > 5 {
        let message = "Delay must be between 1 and 5 seconds.";
        let _ = ctx.reply(message).await?;

        return Ok(());
    }

    let message = format!("Shutting down in {delay} second(s)...");
    let _ = ctx.reply(message).await;

    let manager = ctx.framework().shard_manager.clone();

    let shard_ids = manager
        .runners
        .lock()
        .await
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    for shard_id in shard_ids {
        info!("Shutting down shard {}", shard_id);
        manager.shutdown_finished(shard_id);

        tokio::time::sleep(Duration::from_secs(delay)).await;

        std::process::exit(0);
    }

    Ok(())
}
