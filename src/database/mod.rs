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

use serenity::prelude::TypeMapKey;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Sqlite, Pool};
use sqlx::migrate::MigrateDatabase;
use tokio::time::Instant;
use tracing::{error, debug};
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref DB_URL: String = match dotenvy::var("DATABASE_URL") {
        Ok(url) => url,
        Err(why) => {
            error!("Couldn't find database in environment");
            panic!("{why:?}")
        },
    };
}

pub(crate) struct DatabaseContainer;

impl TypeMapKey for DatabaseContainer {
    type Value = Pool<Sqlite>;
}

pub(crate) async fn initialise_database() -> Pool<Sqlite> {
    create().await;
    let pool = connect().await;

    migrate(&pool).await;
    pool
}

async fn migrate(pool: &Pool<Sqlite>) {
    let start_time = Instant::now();

    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            let elapsed_time = start_time.elapsed();
            debug!("Migrated database in {elapsed_time:?}");
        },
        Err(why) => {
            error!("Couldn't migrate database");
            panic!("{why:?}")
        },
    }
}

async fn connect() -> Pool<Sqlite> {
    let start_time = Instant::now();

    match SqlitePoolOptions::new()
    .max_connections(5)
    .min_connections(1)
    .connect(&DB_URL).await {
        Ok(pool) => {
            let elapsed_time = start_time.elapsed();
            debug!("Connected to database in {elapsed_time:?}");
            pool
        },
        Err(why) => {
            error!("Couldn't connect to database");
            panic!("{why:?}")
        },
    }
}

async fn create() {
    if !exists().await {
        let start_time = Instant::now();

        match Sqlite::create_database(&DB_URL).await {
            Ok(_) => {
                let elapsed_time = start_time.elapsed();
                debug!("Created database in {elapsed_time:?}");
            },
            Err(why) => {
                error!("Couldn't create database");
                panic!("{why:?}")
            },
        }
    }
}

async fn exists() -> bool {
    Sqlite::database_exists(&DB_URL).await.unwrap_or(false)
}