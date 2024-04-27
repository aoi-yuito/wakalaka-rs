// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod accessors;
pub mod builders;
pub mod consts;
pub mod converters;
pub mod envs;
pub mod types;

use poise::Framework;
use serenity::all::GatewayIntents;
use sqlx::PgPool;
use tracing::subscriber;
use types::{Error, SContext, SReady, Throwable};

pub struct Data {
    pub db: PgPool,
}

pub async fn fetch_gateway_intents() -> GatewayIntents {
    GatewayIntents::non_privileged()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
}

pub async fn fetch_user_data(
    _ctx: &SContext,
    _ready: &SReady,
    _framework: &Framework<Data, Error>,
    data: Data,
) -> Throwable<Data> {
    Ok(data)
}

pub async fn build_subscriber() -> Throwable<()> {
    let rust_log = envs::fetch_rust_log_from_env()?;

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(rust_log)
        .compact()
        .finish();
    subscriber::set_global_default(subscriber)?;

    Ok(())
}
