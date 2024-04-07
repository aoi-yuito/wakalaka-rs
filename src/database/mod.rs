// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use wakalaka_core::{
    envs,
    types::{SqlxThrowable, Throwable},
};

pub async fn initialise_db() -> Throwable<SqlitePool> {
    let pool = connect_db().await?;

    migrate_db(&pool).await?;

    Ok(pool)
}

async fn migrate_db(pool: &SqlitePool) -> Throwable<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

async fn connect_db() -> SqlxThrowable<SqlitePool> {
    let options = fetch_connect_options().await;

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options?)
        .await
}

async fn fetch_connect_options() -> SqlxThrowable<SqliteConnectOptions> {
    let db_url = if let Ok(url) = envs::fetch_database_url_from_env() {
        url
    } else {
        format!("sqlite://wakalaka.db")
    };
    let db_filename = db_url.replace("sqlite://", "");

    let connect_options = SqliteConnectOptions::new()
        .filename(db_filename)
        .create_if_missing(true);
    Ok(connect_options)
}
