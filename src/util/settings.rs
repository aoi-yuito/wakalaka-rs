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

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    #[serde(alias = "APPLICATION_ID")]
    pub application_id: i64,

    #[serde(alias = "DISCORD_TOKEN")]
    pub discord_token: String,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        let mut settings = Self::read()?;
        if settings.discord_token.is_empty() {
            if let Ok(token) = env::var("DISCORD_TOKEN") {
                settings.discord_token = token;
                Self::write(&settings)?;
            } else {
                println!("DISCORD_TOKEN not found in environment variables");
                println!("Please enter the Token found in the Developer Portal:");

                let mut token = String::new();
                io::stdin().read_line(&mut token)?;
                token = token.trim().to_owned();

                settings.discord_token = token;
                Self::write(&settings)?;
            }
        }
        Ok(settings)
    }

    fn write(settings: &Self) -> Result<(), Box<dyn error::Error>> {
        if !files::exists("Settings.toml") {
            File::create("Settings.toml")?;
        }

        let contents = toml::to_string(settings)?;
        fs::write("Settings.toml", contents)?;
        Ok(())
    }

    fn read() -> Result<Self, Box<dyn error::Error>> {
        if !files::exists("Settings.toml") {
            let default_settings = Self {
                application_id: 0,
                discord_token: String::new(),
            };
            Self::write(&default_settings)?;
        }

        let contents = fs::read_to_string("Settings.toml")?;
        let value: toml::Value = toml::from_str(&contents)?;

        let authorisation = value
            .get("Authorisation")
            .ok_or("'Authorisation' section not found")?;
        let discord_token = authorisation
            .get("discord_token")
            .ok_or("'discord_token' not found in 'Authorisation' section")?
            .as_str()
            .ok_or("'discord_token' is not a string")?
            .to_owned();
        let application_id = authorisation
            .get("application_id")
            .ok_or("'application_id' not found in 'Authorisation' section")?
            .as_integer()
            .ok_or("'application_id' is not an integer")?;

        let settings = Self {
            application_id,
            discord_token,
        };
        Ok(settings)
    }
}
