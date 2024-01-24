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

use tokio::time::Instant;
use tracing::error;

use crate::{
    check_restricted_guild_channel,
    utility::components::{embeds, replies},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Info",
    guild_only,
    ephemeral
)]
/// Check if yours truly is alive and well.
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let restricted = check_restricted_guild_channel!(ctx);
    if restricted {
        return Ok(());
    }

    let start_time = Instant::now();

    let manager = ctx.framework().shard_manager.clone();
    let runners = manager.runners.lock().await;

    let (mut shard_ids, mut shard_stages, mut shard_latencies) =
        (Vec::new(), Vec::new(), Vec::new());

    for (id, runner) in runners.iter() {
        let stage = runner.stage;
        let latency = runner.latency;

        shard_ids.push(id);
        shard_stages.push(stage);
        shard_latencies.push(latency);
    }

    let elapsed_time = start_time.elapsed();

    let ping_embed = embeds::ping_command_embed(elapsed_time, shard_ids, shard_stages, shard_latencies);

    let reply = replies::reply_embed(ping_embed, true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }
    Ok(())
}
