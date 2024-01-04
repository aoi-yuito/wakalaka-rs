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

use std::marker;

use serde::ser::StdError;
use serenity::{all::GatewayIntents, framework::StandardFramework};
use tracing::Level;
use tracing_subscriber::fmt::Subscriber;
use util::config::Config;

mod commands;
mod events;
mod util;

type Context = serenity::client::Context;
type Error = Box<(dyn StdError + marker::Send + Sync + 'static)>;

#[tokio::main]
pub async fn main() {
    initialise_subscriber();

    let framework = initialise_framework();
    let intents = initialise_intents();
    let config = initialise_config();

    let mut client = initialise_client(config, intents, framework).await;
    client
        .start_autosharded()
        .await
        .expect("An error occurred while running the client");
}

fn initialise_subscriber() {
    Subscriber::builder()
        .with_max_level(Level::INFO)
        .compact()
        .init();
}

fn initialise_framework() -> StandardFramework {
    let framework = StandardFramework::new();
    framework
}

fn initialise_config() -> Config {
    let config = Config::new().expect("An error occurred while reading the config");
    config
}

fn initialise_intents() -> GatewayIntents {
    GatewayIntents::default()
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
}

async fn initialise_client(
    config: Config,
    intents: GatewayIntents,
    framework: StandardFramework,
) -> serenity::Client {
    let token = config.token;

    let client = serenity::Client::builder(token, intents)
        .event_handler(events::Handler)
        .framework(framework)
        .await
        .expect("An error occurred while building the client");
    client
}
