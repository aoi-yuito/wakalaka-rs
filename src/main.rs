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

use ::serenity::all::GatewayIntents;
use poise::Framework;
use sqlx::SqlitePool;
use tokio::time::Instant;
use tracing::{debug, error, level_filters::LevelFilter, subscriber, warn};
use tracing_subscriber::{fmt::Subscriber, EnvFilter};

pub struct Data {
    pub pool: Arc<SqlitePool>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialise_subscriber();

    let pool = database::initialise().await;

    let data = Data {
        pool: Arc::new(pool),
    };

    let token = match dotenvy::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(why) => {
            error!("Couldn't find 'DISCORD_TOKEN' in environment: {why:?}");
            return Err(why.into());
        }
    };
    let intents = framework::initialise_intents();
    let framework = framework::initialise_framework(data).await;

    let mut client = initialise_client(token, intents, framework).await;

    if let Err(why) = client.start_autosharded().await {
        error!("Couldn't start client with automatic sharding: {why:?}");
        return Err(why.into());
    }

    Ok(())
}

async fn initialise_client(
    token: String,
    intents: GatewayIntents,
    framework: Framework<Data, Error>,
) -> serenity::Client {
    let start_time = Instant::now();

    let client = match serenity::Client::builder(token, intents)
        .framework(framework)
        .await
    {
        Ok(client) => client,
        Err(why) => {
            error!("Couldn't initialise client: {why:?}");
            panic!("{why:?}");
        }
    };

    let elapsed_time = start_time.elapsed();
    debug!("Initialised client in {elapsed_time:.2?}");

    client
}

fn initialise_subscriber() {
    let start_time = Instant::now();

    let rust_log = match dotenvy::var("RUST_LOG") {
        Ok(level) => level,
        Err(why) => {
            error!("Couldn't find 'RUST_LOG' in environment: {why:?}");
            panic!("{why:?}");
        }
    };

    let filter = match EnvFilter::try_new(format!("wakalaka_rs={rust_log}")) {
        Ok(filter) => filter,
        Err(_) => {
            error!("Couldn't get filter from environment, setting default...");
            EnvFilter::default()
        }
    };

    let subscriber = Subscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .with_env_filter(filter)
        .compact()
        .finish();
    if let Err(_) = subscriber::set_global_default(subscriber) {
        warn!("Couldn't set custom subscriber, setting default...");

        let default_subscriber = Subscriber::default();
        let _ = subscriber::set_global_default(default_subscriber);
    }

    let elapsed_time = start_time.elapsed();
    debug!("Initialised logger in {elapsed_time:.2?}");
}
