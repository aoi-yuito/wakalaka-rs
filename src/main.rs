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

mod database;
mod framework;
mod utility;

use std::sync::Arc;

use poise::serenity_prelude as serenity;

use ::serenity::all::{GatewayIntents, UserId};
use dashmap::DashMap;
use poise::Framework;
use sqlx::SqlitePool;
use tokio::{sync::Mutex, time::Instant};
use tracing::{debug, error, level_filters::LevelFilter, subscriber, warn};
use tracing_subscriber::{fmt::Subscriber, EnvFilter};

pub struct Data {
    pub pool: Arc<SqlitePool>,
    pub amount_of_messages: Arc<Mutex<DashMap<UserId, (u32, Instant)>>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialise_subscriber()?;

    let pool = database::initialise().await;

    let data = Data {
        pool: Arc::new(pool),
        amount_of_messages: Arc::new(Mutex::new(DashMap::new())),
    };

    let token = initialise_token()?;
    let intents = framework::initialise_intents();
    let framework = framework::initialise_framework(data).await;

    let mut client = initialise_client(token, intents, framework).await?;

    if let Err(why) = client.start_autosharded().await {
        error!("Failed to start client: {why:?}");
        return Err(why.into());
    }

    Ok(())
}

async fn initialise_client(
    token: String,
    intents: GatewayIntents,
    framework: Framework<Data, Error>,
) -> Result<serenity::Client, Error> {
    let start_time = Instant::now();

    let client = match serenity::Client::builder(token, intents)
        .framework(framework)
        .await
    {
        Ok(client) => client,
        Err(why) => {
            error!("Failed to initialise client: {why:?}");
            return Err(why.into());
        }
    };

    let elapsed_time = start_time.elapsed();
    debug!("Initialised client in {elapsed_time:.2?}");

    Ok(client)
}

fn initialise_token() -> Result<String, Error> {
    match dotenvy::var("DISCORD_TOKEN") {
        Ok(token) => Ok(token),
        Err(why) => {
            error!("Failed to find 'DISCORD_TOKEN' in environment: {why:?}");
            return Err(why.into());
        }
    }
}

fn initialise_subscriber() -> Result<(), Error> {
    let start_time = Instant::now();

    let rust_log = match dotenvy::var("RUST_LOG") {
        Ok(level) => level,
        Err(why) => {
            error!("Failed to find 'RUST_LOG' in environment: {why:?}");
            return Err(why.into());
        }
    };

    let filter = match EnvFilter::try_new(format!("wakalaka_rs={rust_log}")) {
        Ok(filter) => filter,
        Err(_) => {
            warn!("Couldn't get filter from environment, using default");
            EnvFilter::default()
        }
    };

    let subscriber = Subscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .with_env_filter(filter)
        .compact()
        .finish();
    if let Err(_) = subscriber::set_global_default(subscriber) {
        warn!("Couldn't set custom subscriber, using default");

        let default_subscriber = Subscriber::default();
        if let Err(why) = subscriber::set_global_default(default_subscriber) {
            error!("Failed to set default subscriber: {why:?}");
            return Err(why.into());
        }
    }

    let elapsed_time = start_time.elapsed();
    debug!("Initialised logger in {elapsed_time:.2?}");

    Ok(())
}
