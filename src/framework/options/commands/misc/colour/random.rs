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

use crate::{
    utility::{
        self,
        components::{embeds, messages},
    },
    Context, Error,
};

#[poise::command(prefix_command, slash_command, category = "Misc", guild_only)]
/// Get information for a random colour.
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let res = client
        .get("https://www.thecolorapi.com/random?format=json")
        .send()
        .await?;
    let res_text = res.text().await?;
    let res_json: serde_json::Value = serde_json::from_str(&res_text)?;

    let hex = res_json["hex"]["clean"].to_string();

    let colour = utility::hex_to_u32(&hex);
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
