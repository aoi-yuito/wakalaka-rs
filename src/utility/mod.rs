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

use tracing::error;

use crate::Error;

pub mod components;
pub mod models;

pub fn rgb_to_u32(code: &String) -> Result<u32, Error> {
    let mut rgb = code.split(',');

    let r = rgb
        .next()
        .expect("Failed to parse red value")
        .parse::<u32>()?;
    let g = rgb
        .next()
        .expect("Failed to parse green value")
        .parse::<u32>()?;
    let b = rgb
        .next()
        .expect("Failed to parse blue value")
        .parse::<u32>()?;

    let hex = format!("{r:02X}{g:02X}{b:02X}");
    Ok(hex_to_u32(&hex))
}

pub fn hex_to_u32(code: &String) -> u32 {
    let hex_code: String = code.chars().filter(|c| c.is_digit(16)).collect();

    match u32::from_str_radix(&hex_code, 16) {
        Ok(colour) => colour,
        Err(why) => {
            error!("Failed to parse {code:?}: {why:?}");
            return 0;
        }
    }
}
