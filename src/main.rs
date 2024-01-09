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

#[macro_use]
mod commands;

mod handlers;
mod util;

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use poise::{serenity_prelude as serenity, Framework, FrameworkOptions};

use ::serenity::all::{ChannelId, GatewayIntents};
use ::serenity::gateway::ShardManager;
use dashmap::DashSet;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tokio::time::Instant;
use tracing::{debug, error, info, subscriber, Level};
use tracing_subscriber::{fmt::Subscriber, EnvFilter};
use util::settings::Settings;

pub struct Data {
    pub suggestion_id: AtomicUsize,
    pub restricted_channels: RwLock<DashSet<ChannelId>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
pub async fn main() {
    initialise_subscriber("debug", Level::DEBUG);

    let (framework, intents, settings) = (
        initialise_framework().await,
        initialise_intents(),
        Settings::new().await,
    );

    let mut client = initialise_client(settings, intents, framework).await;

    let manager = client.shard_manager.clone();
    tokio::spawn(monitor_shards(manager, 300));

    info!("Starting client");
    if let Err(why) = client.start_shards(2).await {
        error!("Couldn't start client: {why:?}");
    }
}

async fn monitor_shards(manager: Arc<ShardManager>, seconds: u64) {
    if seconds < 30 || seconds > 300 {
        error!("Interval must be between 30 and 300 seconds");
        return;
    }

    loop {
        tokio::time::sleep(Duration::from_secs(seconds)).await;

        let runners = manager.runners.lock().await;

        for (id, runner) in runners.iter() {
            let stage = runner.stage;
            let latency = runner.latency;
            debug!("Shard {id} is {stage} with latency of {latency:.2?}");
        }
    }
}

async fn initialise_client(
    settings: Settings,
    intents: GatewayIntents,
    framework: Framework<Data, Error>,
) -> serenity::Client {
    let start_time = Instant::now();

    let token = settings.general.token;

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

fn initialise_intents() -> GatewayIntents {
    let start_time = Instant::now();

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let elapsed_time = start_time.elapsed();
    debug!("Initialised intents in {elapsed_time:.2?}");

    intents
}

async fn initialise_framework() -> Framework<Data, Error> {
    let start_time = Instant::now();

    let framework = Framework::builder()
        .setup(|ctx, _, _| Box::pin(handlers::setup::handle(ctx)))
        .options(FrameworkOptions {
            commands: commands::guild_commands().await,
            post_command: |ctx| Box::pin(handlers::post_command::handle(ctx)),
            event_handler: |ctx, event, framework, data| {
                Box::pin(handlers::event::handle(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .build();

    let elapsed_time = start_time.elapsed();
    debug!("Initialised framework in {elapsed_time:.2?}");

    framework
}

fn initialise_subscriber(crate_level: &'static str, level: Level) {
    let start_time = Instant::now();

    let filter = match EnvFilter::try_new(format!("wakalaka_rs={crate_level}")) {
        Ok(filter) => filter,
        Err(why) => {
            error!("Couldn't get filter from environment variable: {why:?}");
            EnvFilter::default()
        }
    };

    let subscriber = Subscriber::builder()
        .with_max_level(level)
        .with_env_filter(filter)
        .compact()
        .finish();

    match subscriber::set_global_default(subscriber) {
        Ok(_) => (),
        Err(why) => error!("Couldn't set global default for logger: {why:?}"),
    };

    let elapsed_time = start_time.elapsed();
    debug!("Initialised logger in {elapsed_time:.2?}");
}
