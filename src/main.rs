// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod commands;
mod events;
mod framework;
mod options;

use wakalaka_core::{
    envs,
    types::{SClient, Throwable},
    Data,
};

#[tokio::main]
async fn main() -> Throwable<()> {
    wakalaka_core::build_subscriber().await?;

    let data = Data {
        db: wakalaka_db::initialise_db().await?,
    };

    let token = envs::fetch_discord_token_from_env()?;
    let intents = wakalaka_core::fetch_gateway_intents().await;
    let framework = framework::build_framework(data).await;

    let mut client = SClient::builder(token, intents)
        .framework(framework)
        .await?;

    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for CTRL+C");

        manager.shutdown_all().await;
    });

    client.start_autosharded().await?;

    Ok(())
}
