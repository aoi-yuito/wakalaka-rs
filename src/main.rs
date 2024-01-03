/**
 * Copyright (C) 2024 Kasutaja
 *
 * wakalaka-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * wakalaka-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.
 */
mod cogs;
mod core;
mod uses;

use crate::uses::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

#[tokio::main]
pub async fn main() {
    let options = cogs::framework::setup_framework_options().await;

    let framework = cogs::framework::build_framework(options).await;

    let config = match config::Config::new() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };
    let intents = GatewayIntents::default()
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let client = ClientBuilder::new(config.token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
