/**
 * Copyright (C) 2024 Kasutaja
 *
 * wakalaka-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * wakalaka-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::util::uses::*;

const SETTINGS_TOML: &str = "Settings.toml";

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub application_id: i64,
    pub client_id: i64,
    pub token: String,
    pub metadata_channel_id: i64,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let mut settings = Self::read_settings()?;
        if settings.token.is_empty() {
            if let Ok(discord_token) = env::var("DISCORD_TOKEN") {
                settings.token = discord_token;

                Self::write_settings(&settings)?;
            } else {
                println!("DISCORD_TOKEN not found in environment variables");
                println!("Please enter the Token found in the Developer Portal:");

                let mut input_token = String::new();
                io::stdin().read_line(&mut input_token)?;
                input_token = input_token.trim().to_owned();

                settings.token = input_token;
                Self::write_settings(&settings)?;
            }
        }
        Ok(settings)
    }

    pub fn read_settings() -> Result<Self, Box<dyn error::Error>> {
        let application_id = Self::read_section("Developer", "application_id")?
            .parse::<i64>()
            .map_err(|_| "application_id is not an integer")?;
        let client_id = Self::read_section("Developer", "client_id")?
            .parse::<i64>()
            .map_err(|_| "client_id is not an integer")?;
        let token = Self::read_section("Developer", "token")?;

        let metadata_channel_id = Self::read_section("Channels", "metadata_channel_id")?
            .parse::<i64>()
            .map_err(|_| "metadata_channel_id is not an integer")?;

        let settings = Self {
            application_id,
            client_id,
            token,
            metadata_channel_id,
        };
        Ok(settings)
    }

    pub fn write_settings(&self) -> Result<(), Box<dyn error::Error>> {
        if !files::exists(SETTINGS_TOML) {
            File::create(SETTINGS_TOML)?;
        }

        let contents = toml::to_string(&self)?;
        fs::write(SETTINGS_TOML, contents)?;

        Ok(())
    }

    fn read_section(section: &str, key: &'static str) -> Result<String, Box<dyn error::Error>> {
        if !files::exists(SETTINGS_TOML) {
            let default_settings = Self {
                application_id: 0,
                client_id: 0,
                token: String::new(),
                metadata_channel_id: 0,
            };

            Self::write_settings(&default_settings)?;
        }

        let contents = fs::read_to_string(SETTINGS_TOML)?;
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
