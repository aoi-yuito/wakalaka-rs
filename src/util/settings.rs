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

use std::path::PathBuf;

use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, File},
    io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    time::Instant,
};
use tracing::{debug, error};

#[derive(Serialize, Deserialize)]
pub(crate) struct Settings {
    #[serde(rename = "General")]
    pub(crate) general: General,
}

impl Settings {
    pub(crate) async fn new() -> Self {
        let data_directory = data_directory();
        if !data_directory
            .join(".wakalaka")
            .join("Wakalaka.toml")
            .exists()
        {
            let (application_id, token) = (
                prompt_for_application_id()
                    .await
                    .expect("No valid application ID found"),
                prompt_for_token().await.expect("No valid token found"),
            );

            match Self::write_to_toml(application_id, token).await {
                Ok(value) => value,
                Err(why) => {
                    error!("Couldn't write to TOML: {why:?}");
                    panic!("{why:?}");
                }
            }
        } else {
            match Self::read_from_toml().await {
                Ok(value) => value,
                Err(why) => {
                    error!("Couldn't read from TOML: {why:?}");
                    panic!("{why:?}");
                }
            }
        }
    }

    async fn write_to_toml(application_id: u64, token: String) -> Result<Self, crate::Error> {
        let start_time = Instant::now();

        let mut data_directory = data_directory();
        data_directory.push(".wakalaka");

        if !data_directory.exists() {
            fs::create_dir_all(&data_directory).await?;
        }
        data_directory.push("Wakalaka.toml");

        let settings = Self {
            general: General {
                application_id,
                token,
            },
        };

        let toml = toml::to_string(&settings)?;

        let mut file = File::create(data_directory).await?;
        file.write_all(toml.as_bytes()).await?;

        let elapsed_time = start_time.elapsed();
        debug!("Wrote settings to TOML in {elapsed_time:?}");

        Ok(settings)
    }

    async fn read_from_toml() -> Result<Self, crate::Error> {
        let start_time = Instant::now();

        let mut data_directory = data_directory();
        data_directory.push(".wakalaka");
        data_directory.push("Wakalaka.toml");

        let mut file = File::open(data_directory).await?;
        let mut file_buffer = String::new();
        file.read_to_string(&mut file_buffer).await?;

        let settings = toml::from_str(&file_buffer)?;

        let elapsed_time = start_time.elapsed();
        debug!("Read settings from TOML in {elapsed_time:?}");

        Ok(settings)
    }
}

fn data_directory() -> PathBuf {
    let data_directory = match dirs::data_dir() {
        Some(data_directory) => data_directory,
        None => {
            let mut manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            manifest_directory.push("data");
            manifest_directory
        }
    };
    data_directory
}

#[derive(Serialize, Deserialize)]
pub(crate) struct General {
    pub(crate) application_id: u64,
    pub(crate) token: String,
}

async fn prompt_for_token() -> Result<String, crate::Error> {
    println!("Enter token from Discord Developer Portal:");

    let mut stdin = BufReader::new(io::stdin());

    let mut token = String::new();
    stdin.read_line(&mut token).await?;

    let token_re = Regex::new(r"([A-Za-z0-9])+\.(.{6})\.([A-Za-z0-9])+")?;
    while !token_re.is_match(&token) {
        println!("Invalid token. Please try again.");

        token.clear();
        stdin.read_line(&mut token).await?;
    }

    Ok(token.trim().to_string())
}

async fn prompt_for_application_id() -> Result<u64, crate::Error> {
    println!("Enter application ID from Discord Developer Portal:");

    let mut stdin = BufReader::new(io::stdin());

    let mut application_id = String::new();
    stdin.read_line(&mut application_id).await?;

    let application_id_re = Regex::new(r"([0-9]{19})")?;
    while !application_id_re.is_match(&application_id) {
        println!("Invalid application ID. Please try again.");

        application_id.clear();
        stdin.read_line(&mut application_id).await?;
    }

    Ok(application_id.trim().parse::<u64>()?)
}
