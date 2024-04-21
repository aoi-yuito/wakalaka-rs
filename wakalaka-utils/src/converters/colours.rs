// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::error;
use wakalaka_core::types::Throwable;

pub fn rgba_to_hex(code: &String) -> Throwable<u32> {
    let code = code.trim();

    let mut rgba_code = code.split(',');

    let r = rgba_code.next().unwrap_or("0").parse::<u32>()?;
    let g = rgba_code.next().unwrap_or("0").parse::<u32>()?;
    let b = rgba_code.next().unwrap_or("0").parse::<u32>()?;
    let a = rgba_code.next().unwrap_or("0").parse::<u32>()?;

    let hex_code = format!("{r:02X}{g:02X}{b:02X}{a:02X}");

    let rr = hex_code
        .chars()
        .nth(0)
        .expect("Failed to get chars representing Red");
    let gg = hex_code
        .chars()
        .nth(1)
        .expect("Failed to get chars representing Green");
    let bb = hex_code
        .chars()
        .nth(2)
        .expect("Failed to get chars representing Blue");
    let aa = hex_code
        .chars()
        .nth(3)
        .expect("Failed to get chars representing Alpha");

    Ok(
        u32::from_str_radix(&format!("{rr}{gg}{bb}{aa}"), 16).unwrap_or_else(|e| {
            error!("Failed to parse {code}: {e:?}");
            0
        }),
    )
}

pub fn rgb_to_hex(code: &String) -> Throwable<u32> {
    let code = code.trim();

    let mut rgb_code = code.split(',');
    let r = rgb_code.next().unwrap_or("0").parse::<u32>()?;
    let g = rgb_code.next().unwrap_or("0").parse::<u32>()?;
    let b = rgb_code.next().unwrap_or("0").parse::<u32>()?;

    let hex_code = format!("{r:02X}{g:02X}{b:02X}");
    let rr = hex_code
        .chars()
        .nth(0)
        .expect("Failed to get chars representing Red");
    let gg = hex_code
        .chars()
        .nth(1)
        .expect("Failed to get chars representing Green");
    let bb = hex_code
        .chars()
        .nth(2)
        .expect("Failed to get chars representing Blue");

    Ok(
        u32::from_str_radix(&format!("{rr}{gg}{bb}"), 16).unwrap_or_else(|e| {
            error!("Failed to parse {code}: {e:?}");
            0
        }),
    )
}
