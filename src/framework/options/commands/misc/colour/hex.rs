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

use crate::{
    utility::{
        self,
        components::{embeds, messages, replies},
    },
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Miscellaneous",
    required_bot_permissions = "SEND_MESSAGES",
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
    colour = colour
        .trim_start_matches('#')
        .trim_start_matches("0x")
        .to_string()
        .to_ascii_uppercase();

    let hex_regex = Regex::new(r"^[0-9a-fA-F]{3}$|^[0-9a-fA-F]{6}$").unwrap();
    if !hex_regex.is_match(&colour) {
        let reply = messages::error_reply(None, "Colour must be in hexadecimal format!", true);
        ctx.send(reply).await?;

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
    let hex_colour = format!("{:06X}", colour);
    let colour_url = format!("https://singlecolorimage.com/get/{hex_colour}/400x400");

    let embed = embeds::colour_command_embed(colour, &colour_url, &res_json);

    let reply = replies::reply_embed(embed, false);
    ctx.send(reply).await?;

    Ok(())
}
