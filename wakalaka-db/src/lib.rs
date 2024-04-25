// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod checks;
pub mod queries;

use std::str::FromStr;

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use wakalaka_core::{
    envs,
    types::{SqlxThrowable, Throwable},
};

pub async fn initialise_db() -> Throwable<PgPool> {
    let pool = connect_db().await?;

    migrate_db(&pool).await?;

    Ok(pool)
}

async fn migrate_db(pool: &PgPool) -> Throwable<()> {
    sqlx::migrate!("../migrations").run(pool).await?;
    Ok(())
}

async fn connect_db() -> SqlxThrowable<PgPool> {
    let options = fetch_connect_options().await;

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options?)
        .await
}

async fn fetch_connect_options() -> SqlxThrowable<PgConnectOptions> {
    let db_url = if let Ok(url) = envs::fetch_database_url_from_env() {
        url
    } else {
        format!("postgresql://postgres:password@localhost/wakalaka")
    };

    let connect_options = PgConnectOptions::from_str(&db_url)?;
    Ok(connect_options)
}