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
    utility::{
        self,
        components::{embeds, messages},
    },
    Context, Error,
};

#[poise::command(prefix_command, slash_command, category = "Misc", guild_only)]
/// Get information for a colour from RGB representation.
pub(crate) async fn rgb(
    ctx: Context<'_>,
    #[description = "The colour in RGB."]
    #[min_length = 5]
    #[max_length = 11]
    colour: String,
) -> Result<(), Error> {
    let rgb_re = Regex::new(r"^\d{1,3},\d{1,3},\d{1,3}$").unwrap();
    if !rgb_re.is_match(&colour) {
        let reply = messages::error_reply("Sorry, but that's not a valid RGB colour.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "https://www.thecolorapi.com/id?rgb={colour}&format=json"
        ))
        .send()
        .await?;
    let res_text = res.text().await?;
    let res_json: serde_json::Value = serde_json::from_str(&res_text)?;

    let colour = utility::rgb_to_u32(&colour);
    let hex_colour = format!("{:06X}", colour);
    let colour_url = format!("https://singlecolorimage.com/get/{hex_colour}/400x400");

    let embed = embeds::colour_embed(colour, &colour_url, &res_json);

    let reply = messages::reply_embed(embed, false);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
