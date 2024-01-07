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

use serde::{ Deserialize, Serialize };
use tokio::{ fs::{ File, self }, io::{ AsyncReadExt, AsyncWriteExt } };
use tracing::log::error;

use super::files;

const WAKALAKA_DIRECTORY: &str = ".wakalaka";
const WAKALAKA_TOML: &str = "Wakalaka.toml";

pub(crate) struct Config {
    pub(crate) general: General,
}

impl Config {
    pub(crate) async fn new() -> Self {
        let general = General::new().await;
        Self {
            general,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct General {
    pub(crate) application_id: u64,
    pub(crate) token: String,
}

impl General {
    pub(crate) async fn new() -> Self {
        if
            !dirs
                ::data_dir()
                .expect("Error while getting data directory")
                .join(WAKALAKA_DIRECTORY)
                .join(WAKALAKA_TOML)
                .exists()
        {
            let token = prompt_token();
            let application_id = prompt_application_id();
            Self::write_to_toml(application_id, token).await
        } else {
            Self::read_from_toml().await
        }
    }

    async fn write_to_toml(application_id: u64, token: String) -> Self {
        let mut data_dir = dirs::data_dir().expect("Error while getting data directory");
        data_dir.push(".wakalaka");

        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).await.unwrap_or_else(|why| {
                error!("Error while creating data directory: {why}");
                panic!("{why:?}");
            });
        }
        data_dir.push("Wakalaka.toml");

        let general = General {
            application_id,
            token,
        };

        let toml = toml::to_string(&general).expect("Error while serialising TOML");

        let mut file = File::create(data_dir).await.unwrap_or_else(|why| {
            error!("Error while creating file: {why}");
            panic!("{why:?}");
        });
        file.write_all(toml.as_bytes()).await.unwrap_or_else(|why| {
            error!("Error while writing to false: {why}");
            panic!("{why:?}");
        });

        general
    }

    async fn read_from_toml() -> Self {
        let mut data_dir = dirs::data_dir().expect("Error while getting data directory");
        data_dir.push(".wakalaka");
        data_dir.push("Wakalaka.toml");

        let mut file = File::open(data_dir).await.unwrap_or_else(|why| {
            error!("Error while opening file: {why}");
            panic!("{why:?}");
        });

        let mut toml = String::new();
        file.read_to_string(&mut toml).await.unwrap_or_else(|why| {
            error!("Error while reading file: {why}");
            panic!("{why:?}");
        });

        let general = toml::from_str(&toml).expect("Error while deserialising TOML");
        general
    }
}

fn prompt_token() -> String {
    println!("Enter token from Discord Developer Portal:");
    let mut token = String::new();
    std::io::stdin().read_line(&mut token).expect("Error while reading token");
    format!("{}", token.trim())
}

fn prompt_application_id() -> u64 {
    println!("Enter application ID from Discord Developer Portal:");
    let mut application_id = String::new();
    std::io::stdin().read_line(&mut application_id).expect("Error while reading application ID");
    let parsed_application_id = application_id
        .trim()
        .parse::<u64>()
        .unwrap_or_else(|why| {
            error!("Error while parsing application ID: {why}");
            panic!("{why:?}");
        });
    parsed_application_id
}
