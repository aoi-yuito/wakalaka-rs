// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::error;

use crate::Throwable;

pub(crate) fn database_url() -> Throwable<String> {
    let database_url = match dotenvy::var("DATABASE_URL") {
        Ok(url) => url,
        Err(why) => {
            error!("DATABASE_URL not found in environment: {why:?}");
            return Err(why.into());
        }
    };
    Ok(database_url)
}

pub(crate) fn discord_token() -> Throwable<String> {
    let discord_token = match dotenvy::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(why) => {
            error!("DISCORD_TOKEN not found in environment: {why:?}");
            return Err(why.into());
        }
    };
    Ok(discord_token)
}

pub(crate) fn rust_log() -> Throwable<String> {
    let rust_log = match dotenvy::var("RUST_LOG") {
        Ok(level) => level,
        Err(why) => {
            error!("RUST_LOG not found in environment: {why:?}");
            return Err(why.into());
        }
    };
    Ok(rust_log)
}
