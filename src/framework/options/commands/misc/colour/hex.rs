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
    check_restricted_guild_channel,
    utility::{
        self,
        components::{embeds, messages, replies},
    },
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Misc",
    guild_only,
    user_cooldown = 5
)]
/// Get information for a colour from hexadecimal representation.
pub async fn hex(
    ctx: Context<'_>,
    #[description = "The colour in hexadecimal."]
    #[min_length = 3]
    #[max_length = 8] // This is to allow users to use `#` or `0x` as a prefix.
    mut colour: String,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    colour = colour
        .trim_start_matches('#')
        .trim_start_matches("0x")
        .to_string()
        .to_ascii_uppercase();

    let hex_regex = Regex::new(r"^[0-9a-fA-F]{3}$|^[0-9a-fA-F]{6}$").unwrap();
    if !hex_regex.is_match(&colour) {
        let reply =
            messages::error_reply("Sorry, but that's not a valid hexadecimal colour.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "https://www.thecolorapi.com/id?hex={colour}&format=json"
        ))
        .send()
        .await?;
    let res_text = res.text().await?;
    let res_json: serde_json::Value = serde_json::from_str(&res_text)?;

    let colour = utility::hex_to_u32(&colour);
    let colour_url = format!("https://singlecolorimage.com/get/{colour}/400x400");

    let embed = embeds::colour_command_embed(colour, &colour_url, &res_json);

    let reply = replies::reply_embed(embed, false);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
