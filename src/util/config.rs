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
use tracing::log::error;

use super::files;

const CONFIG_TOML: &str = "Config.toml";

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub application_id: u64, // Same as Client ID
    pub token: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let mut config = Self::read_config()?;
        if config.token.is_empty() || config.application_id == 0 {
            if let Ok(discord_token) = env::var("DISCORD_TOKEN") {
                config.token = discord_token;
            } else {
                error!("DISCORD_TOKEN not found in environment variable");

                println!("Please enter Token from Discord Developer Portal:");
                let mut input_token = String::new();
                io::stdin().read_line(&mut input_token)?;
                input_token = input_token.trim().to_owned();

                config.token = input_token;
            }

            if let Ok(application_id) = env::var("APPLICATION_ID") {
                config.application_id = application_id.parse::<u64>().unwrap_or(0);
            } else {
                error!("APPLICATION_ID not found in environment variable");

                println!("Please enter Application ID from Discord Developer Portal:");
                let mut input_id = String::new();
                io::stdin().read_line(&mut input_id)?;
                input_id = input_id.trim().to_owned();

                config.application_id = input_id.parse::<u64>().unwrap_or(0);
            }

            Self::write_config(&config)?;
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
        const GENERAL_SECTION: &str = "General";

        let application_id = Self::read_section(GENERAL_SECTION, "application_id")?
            .parse::<u64>()
            .map_err(|why| format!("An error occurred while parsing Application ID: {why}"))?;
        let token = Self::read_section(GENERAL_SECTION, "token")
            .map_err(|why| format!("An error occurred while reading Token: {why}"))?;

        let config = Self {
            application_id,
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

        let section = value.get(section).ok_or(format!("{section} not found"))?;
        let key = section
            .get(key)
            .ok_or(format!("{key} not found in {section}"))?;

        let value = match key {
            toml::Value::String(s) => s.clone(),
            _ => key.to_string(),
        };
        Ok(value)
    }
}
