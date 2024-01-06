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

mod commands;
mod events;
mod util;

use serenity::{ all::GatewayIntents, framework::StandardFramework };
use tracing::Level;
use tracing_subscriber::{ fmt::Subscriber, EnvFilter };
use util::config::Config;

type Context = serenity::client::Context;

#[tokio::main]
pub async fn main() {
    initialise_subscriber();

    let framework = initialise_framework();
    let intents = initialise_intents();
    let config = initialise_config();

    let mut client = initialise_client(config, intents, framework).await;
    client.start_autosharded().await.expect("Error while running client");
}

fn initialise_subscriber() {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("wakalaka_rs=info"))
        .expect("Error while creating filter");

    Subscriber::builder().with_max_level(Level::INFO).with_env_filter(filter).compact().init();
}

fn initialise_framework() -> StandardFramework {
    let framework = StandardFramework::new();
    framework
}

fn initialise_config() -> Config {
    let config = Config::new().expect("Error while reading config");
    config
}

fn initialise_intents() -> GatewayIntents {
    GatewayIntents::default() |
        GatewayIntents::GUILDS |
        GatewayIntents::GUILD_MEMBERS |
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT
}

async fn initialise_client(
    config: Config,
    intents: GatewayIntents,
    framework: StandardFramework
) -> serenity::Client {
    let token = config.token;

    let client = serenity::Client
        ::builder(token, intents)
        .event_handler(events::Handler)
        .framework(framework).await
        .expect("Error while building client");
    client
}
