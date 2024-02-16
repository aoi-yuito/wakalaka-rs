// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub(crate) mod components;
pub(crate) mod environment;
pub(crate) mod models;

use tracing::error;

use crate::Error;

pub(crate) const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const CARGO_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) const CARGO_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub(crate) const CARGO_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub(crate) const CARGO_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub(crate) const GITHUB_URL: &str = "https://github.com/Kawaxte";

pub(crate) const INVITE_URL: &str = "https://discord.gg/jUZVWk7q2q";
pub(crate) const BOT_INVITE_URL: &str = "https://discord.com/api/oauth2/authorize?client_id=1190718691055251548&permissions=9899241204854&scope=bot";

pub(crate) fn rgb_to_u32(code: &String) -> Result<u32, Error> {
    let mut rgb = code.split(',');

    let r = rgb.next().unwrap_or("0").parse::<u32>()?;
    let g = rgb.next().unwrap_or("0").parse::<u32>()?;
    let b = rgb.next().unwrap_or("0").parse::<u32>()?;

    let hex = format!("{r:02X}{g:02X}{b:02X}");
    Ok(hex_to_u32(&hex))
}

pub(crate) fn hex_to_u32(code: &String) -> u32 {
    let hex_code: String = code.chars().filter(|c| c.is_digit(16)).collect();

    match u32::from_str_radix(&hex_code, 16) {
        Ok(colour) => colour,
        Err(why) => {
            error!("Failed to parse {code:?}: {why:?}");
            0
        }
    }
}
