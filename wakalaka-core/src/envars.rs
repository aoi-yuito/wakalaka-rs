// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::error;

use crate::types::Throwable;

pub fn database_url() -> Throwable<String> {
    let database_url = dotenvy::var("DATABASE_URL").map_err(|why| {
        error!("Failed to find DATABASE_URL in environment: {why:?}");
        why
    })?;
    Ok(database_url)
}

pub fn discord_token() -> Throwable<String> {
    let discord_token = dotenvy::var("DISCORD_TOKEN").map_err(|why| {
        error!("Failed to find DISCORD_TOKEN in environment: {why:?}");
        why
    })?;
    Ok(discord_token)
}

pub fn rust_log() -> Throwable<String> {
    let rust_log = dotenvy::var("RUST_LOG").map_err(|why| {
        error!("Failed to find RUST_LOG in environment: {why:?}");
        why
    })?;
    Ok(rust_log)
}
