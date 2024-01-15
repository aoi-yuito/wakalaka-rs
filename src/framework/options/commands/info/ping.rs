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

use poise::CreateReply;
use serenity::{
    all::{colours::branding, ShardId},
    builder::CreateEmbed,
    gateway::ConnectionStage,
};
use tokio::time::{Duration, Instant};

use crate::{Context, Error};

/// Checks if yours truly is alive and well.
#[poise::command(slash_command)]
pub(crate) async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start_time = Instant::now();

    let manager = ctx.framework().shard_manager.clone();
    let runners = manager.runners.lock().await;
    for (id, runner) in runners.iter() {
        let stage = runner.stage;
        let latency = runner.latency;

        let elapsed_time = start_time.elapsed();

        let embed = embed(elapsed_time, id, stage, latency);

        let reply = CreateReply::default().embed(embed);
        let _ = ctx.send(reply).await;
    }
    Ok(())
}

fn embed(
    elapsed_time: Duration,
    shard_id: &ShardId,
    stage: ConnectionStage,
    latency: Option<Duration>,
) -> CreateEmbed {
    if latency.is_some() {
        let latency = match latency {
            Some(value) => value.as_millis(),
            None => 0,
        };

        CreateEmbed::default()
            .title("Pong!")
            .field(
                "Shards",
                format!("{shard_id} ({stage}, {latency:.2?}ms)"),
                true,
            )
            .field("Response", format!("{elapsed_time:.2?}"), true)
            .colour(branding::BLURPLE)
    } else {
        CreateEmbed::default()
            .title("Pong!")
            .field("Shards", format!("{shard_id} ({stage})"), true)
            .field("Response", format!("{elapsed_time:.2?}"), true)
            .colour(branding::BLURPLE)
    }
}
