// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod database;
mod framework;
mod integrations;
mod utils;

use ::serenity::all::GatewayIntents;
use poise::serenity_prelude as serenity;
use sqlx::SqlitePool;
use tracing::subscriber;
use utils::environment;

pub(crate) struct Data {
    pub(crate) db: SqlitePool,
}

type Context<'a> = poise::Context<'a, Data, Error>;
type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, Error>;

type SClient = serenity::Client;
type SContext = serenity::Context;
type SReady = serenity::Ready;

type Error = Box<dyn std::error::Error + Send + Sync>;
type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;
type SqlxError = sqlx::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let filter = environment::rust_log()?;

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .compact()
        .finish();
    subscriber::set_global_default(subscriber)?;

    let data = Data {
        db: database::start().await?,
    };

    let token = environment::discord_token()?;
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let framework = framework::framework(data).await;

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
