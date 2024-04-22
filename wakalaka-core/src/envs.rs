// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::types::Throwable;

pub fn fetch_database_url_from_env() -> Throwable<String> {
    let database_url = dotenvy::var("DATABASE_URL").map_err(|e| {
        tracing::error!("Failed to find DATABASE_URL in environment: {e:?}");
        e
    })?;
    Ok(database_url)
}

pub fn fetch_discord_token_from_env() -> Throwable<String> {
    let discord_token = dotenvy::var("DISCORD_TOKEN").map_err(|e| {
        tracing::error!("Failed to find DISCORD_TOKEN in environment: {e:?}");
        e
    })?;
    Ok(discord_token)
}

pub fn fetch_rust_log_from_env() -> Throwable<String> {
    let rust_log = dotenvy::var("RUST_LOG").map_err(|e| {
        tracing::error!("Failed to find RUST_LOG in environment: {e:?}");
        e
    })?;
    Ok(rust_log)
}
