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

pub(super) mod components;
pub(super) mod models;

pub fn rgb_to_u32(code: &String) -> u32 {
    let mut rgb = code.split(',');

    let r = rgb.next().unwrap().parse::<u32>().unwrap();
    let g = rgb.next().unwrap().parse::<u32>().unwrap();
    let b = rgb.next().unwrap().parse::<u32>().unwrap();

    let hex = format!("{r:02X}{g:02X}{b:02X}");
    hex_to_u32(&hex)
}

pub fn hex_to_u32(code: &String) -> u32 {
    let hex_code: String = code.chars().filter(|c| c.is_digit(16)).collect();

    match u32::from_str_radix(&hex_code, 16) {
        Ok(colour) => colour,
        Err(why) => {
            error!("Couldn't parse {code}: {why:?}");
            return 0;
        }
    }
}
