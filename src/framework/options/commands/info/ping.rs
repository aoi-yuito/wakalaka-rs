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
    utility::components::{embeds, messages},
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
pub(crate) async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start_time = Instant::now();

    let manager = ctx.framework().shard_manager.clone();
    let runners = manager.runners.lock().await;
    for (id, runner) in runners.iter() {
        let stage = runner.stage;
        let latency = runner.latency;

        let elapsed_time = start_time.elapsed();

        let ping_embed = embeds::ping_embed(elapsed_time, id, stage, latency);

        let reply = messages::reply_embed(ping_embed, true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }
    }
    Ok(())
}
