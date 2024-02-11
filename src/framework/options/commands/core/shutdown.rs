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
/// Put yours truly to sleep.
pub async fn shutdown(
    ctx: Context<'_>,
    #[description = "The amount of seconds to wait before shutting down."]
    #[min = 1]
    #[max = 5]
    duration: u64,
) -> Result<(), Error> {
    let reply = if duration == 1 {
        messages::reply(format!("Going to sleep in {duration} second..."), true)
    } else {
        messages::reply(format!("Going to sleep in {duration} seconds..."), true)
    };
    ctx.send(reply).await?;

    let manager = Arc::new(ctx.framework().shard_manager);
    manager.shutdown_all().await;

    tokio::time::sleep(Duration::from_secs(duration)).await;

    std::process::exit(0);
}
