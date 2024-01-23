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

#[macro_use]
pub mod guild_channels;
pub mod guild_members;
pub mod guilds;
pub mod infractions;
pub mod suggestions;
pub mod users;

use lazy_static::lazy_static;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tokio::time::Instant;
use tracing::{debug, error, info};

lazy_static! {
    pub static ref DB_URL: String = match dotenvy::var("DATABASE_URL") {
        Ok(url) => url,
        Err(why) => {
            error!("Couldn't find database in environment: {why:?}");
            panic!("{why:?}")
        }
    };
}

pub async fn initialise() -> SqlitePool {
    let start_time = Instant::now();

    let pool = connect().await.unwrap();

    match migrate(&pool).await {
        Ok(_) => (),
        Err(why) => {
            error!("Couldn't migrate database: {why:?}");
            panic!("{why:?}")
        }
    }

    let elapsed_time = start_time.elapsed();
    debug!("Initialised database in {elapsed_time:.2?}");

    pool
}

async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let start_time = Instant::now();

    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            let elapsed_time = start_time.elapsed();
            debug!("Migrated database in {elapsed_time:.2?}");
            Ok(())
        }
        Err(why) => {
            error!("Couldn't migrate database: {why:?}");
            Err(why.into())
        }
    }
}

async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let start_time = Instant::now();

    match SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&DB_URL)
        .await
    {
        Ok(pool) => {
            let elapsed_time = start_time.elapsed();
            info!("Connected to database in {elapsed_time:.2?}");
            Ok(pool)
        }
        Err(why) => {
            error!("Couldn't connect to database: {why:?}");
            Err(why)
        }
    }
}
