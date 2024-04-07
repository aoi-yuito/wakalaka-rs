// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod consts;
pub mod envs;
pub mod types;

use sqlx::SqlitePool;

pub struct Data {
    pub db: SqlitePool,
}
