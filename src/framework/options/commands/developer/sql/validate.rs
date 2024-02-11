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

use crate::{
    framework::commands::developer::SQL_VALIDATOR_URL,
    utility::components::{embeds, messages, replies},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Developer",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Check SQL query for syntax errors.
pub async fn validate(
    ctx: Context<'_>,
    #[description = "The query to validate."] query: String,
) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let res = client
        .post(SQL_VALIDATOR_URL)
        .body(query.clone())
        .send()
        .await?;
    let res_text = res.text().await?;
    let res_json: serde_json::Value = serde_json::from_str(&res_text)?;

    let error = res_json
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|obj| {
                    let message = obj.get("m").map(|m| m.as_str()).flatten();
                    let token = obj.get("t").map(|t| t.as_str()).flatten();
                    let line = obj.get("l").map(|l| l.as_u64()).flatten();
                    let column = obj.get("c").map(|c| c.as_u64()).flatten();
                    (message, token, line, column)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if !error.is_empty() {
        for (message, token, line, column) in error {
            if let Some(message) = message {
                if let Some(token) = token {
                    if let Some(line) = line {
                        if let Some(column) = column {
                            let embed = embeds::sql_validate_command_embed(
                                &query,
                                &format!("{}", message),
                                &format!("{}", token),
                                &line,
                                &column,
                            );

                            let reply = replies::reply_embed(embed, false);
                            ctx.send(reply).await?;
                        }
                    }
                }
            }
        }
    } else {
        let reply = messages::ok_reply(None, "Your SQL query is valid!", false);
        ctx.send(reply).await?;
    }

    Ok(())
}
