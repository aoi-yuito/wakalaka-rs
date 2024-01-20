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

use regex::Regex;
use tracing::error;

use crate::{
    utility::{embeds, messages},
    Context, Error,
};

/// Get colour information.
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("hex", "rgb"),
    category = "Misc",
    guild_only
)]
pub(crate) async fn colour(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Get colour information from RGB representation.
#[poise::command(prefix_command, slash_command, category = "Misc", guild_only)]
pub(crate) async fn rgb(
    ctx: Context<'_>,
    #[description = "The colour in RGB."] code: String,
) -> Result<(), Error> {
    let rgb_re = Regex::new(r"^\d{1,3},\d{1,3},\d{1,3}$").unwrap();
    if !rgb_re.is_match(&code) {
        let reply = messages::error_reply("Colour code must be in RGB.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }
        return Ok(());
    }

    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "https://www.thecolorapi.com/id?rgb={code}&format=json"
        ))
        .send()
        .await?;
    let res_text = res.text().await?;
    let res_json: serde_json::Value = serde_json::from_str(&res_text)?;

    let colour = rgb_to_u32(&code);
    let hex_colour = format!("{:06X}", colour);

    let colour_url = format!("https://singlecolorimage.com/get/{hex_colour}/400x400");

    let embed = embeds::colour_embed(colour, &colour_url, &res_json);

    let reply = messages::reply_embed(embed, false);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
    }

    Ok(())
}

/// Get colour information from hexadecimal representation.
#[poise::command(prefix_command, slash_command, category = "Misc", guild_only)]
pub(crate) async fn hex(
    ctx: Context<'_>,
    #[description = "The colour in hexadecimal."] mut code: String,
) -> Result<(), Error> {
    code = code
        .trim_start_matches('#')
        .trim_start_matches("0x")
        .to_string()
        .to_ascii_uppercase();

    let hex_regex = Regex::new(r"^[0-9a-fA-F]{3}$|^[0-9a-fA-F]{6}$").unwrap();
    if !hex_regex.is_match(&code) {
        let reply = messages::error_reply("Colour code must be in hexadecimal.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }
        return Ok(());
    }

    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "https://www.thecolorapi.com/id?hex={code}&format=json"
        ))
        .send()
        .await?;
    let res_text = res.text().await?;
    let res_json: serde_json::Value = serde_json::from_str(&res_text)?;

    let colour = hex_to_u32(&code);

    let colour_url = format!("https://singlecolorimage.com/get/{code}/400x400");

    let embed = embeds::colour_embed(colour, &colour_url, &res_json);

    let reply = messages::reply_embed(embed, false);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
    }

    Ok(())
}

fn rgb_to_u32(code: &String) -> u32 {
    let mut rgb = code.split(',');

    let r = rgb.next().unwrap().parse::<u32>().unwrap();
    let g = rgb.next().unwrap().parse::<u32>().unwrap();
    let b = rgb.next().unwrap().parse::<u32>().unwrap();

    let hex = format!("{r:02X}{g:02X}{b:02X}");
    hex_to_u32(&hex)
}

fn hex_to_u32(code: &String) -> u32 {
    match u32::from_str_radix(&code, 16) {
        Ok(colour) => colour,
        Err(_) => {
            error!("Couldn't parse hex code: {code}");
            return 0;
        }
    }
}
