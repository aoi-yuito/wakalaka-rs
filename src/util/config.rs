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

use serde::{Deserialize, Serialize};
use std::{fs::File, *};

use super::files;

const CONFIG_TOML: &str = "Config.toml";

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub application_id: u64,
    pub client_id: u64,
    pub token: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let mut config = Self::read_config()?;
        if config.token.is_empty() {
            if let Ok(discord_token) = env::var("DISCORD_TOKEN") {
                config.token = discord_token;

                Self::write_config(&config)?;
            } else {
                println!("'DISCORD_TOKEN' not found in environment variables");
                println!("Please enter the Token found in the Discord Developer Portal:");

                let mut input_token = String::new();
                io::stdin().read_line(&mut input_token)?;
                input_token = input_token.trim().to_owned();

                config.token = input_token;
                Self::write_config(&config)?;
            }
        }
        Ok(config)
    }

    pub fn write_config(&self) -> Result<(), Box<dyn error::Error>> {
        if !files::exists(CONFIG_TOML) {
            File::create(CONFIG_TOML)?;
        }

        let contents = toml::to_string(&self)?;
        fs::write(CONFIG_TOML, contents)?;

        Ok(())
    }

    pub fn read_config() -> Result<Self, Box<dyn error::Error>> {
        let application_id = Self::read_section("Developer", "application_id")?
            .parse::<u64>()
            .map_err(|_| "application_id is not an integer")?;
        let client_id = Self::read_section("Developer", "client_id")?
            .parse::<u64>()
            .map_err(|_| "client_id is not an integer")?;
        let token = Self::read_section("Developer", "token")?;

        let config = Self {
            application_id,
            client_id,
            token,
        };
        Ok(config)
    }

    fn read_section(section: &str, key: &'static str) -> Result<String, Box<dyn error::Error>> {
        if !files::exists(CONFIG_TOML) {
            let default_config = Self::default();

            Self::write_config(&default_config)?;
        }

        let contents = fs::read_to_string(CONFIG_TOML)?;
        let value: toml::Value = toml::from_str(&contents)?;

        let section = value
            .get(section)
            .ok_or(format!("'{}' section not found", section))?;
        let key = section
            .get(key)
            .ok_or(format!("'{}' not found in '{}' section", key, section))?;

        let value = match key {
            toml::Value::String(s) => s.clone(),
            _ => key.to_string(),
        };
        Ok(value)
    }
}