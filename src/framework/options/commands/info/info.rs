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
    check_restricted_guild_channel,
    utility::components::{embeds, replies},
    Context, Error,
};

use super::{
    CARGO_AUTHORS, CARGO_DESCRIPTION, CARGO_NAME, CARGO_RUST_VERSION, CARGO_VERSION, GITHUB_URL,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Info",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5,
    ephemeral
)]
/// Get basic information about yours truly.
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let http = ctx.http();

    let bot_raw = match http.get_current_user().await {
        Ok(value) => value,
        Err(why) => {
            error!("Couldn't get current user: {why:?}");
            return Err(why.into());
        }
    };
    let bot_avatar_url = bot_raw.avatar_url().unwrap_or(bot_raw.default_avatar_url());

    let constants = [
        CARGO_NAME,         // 0
        CARGO_VERSION,      // 1
        CARGO_AUTHORS,      // 2
        CARGO_DESCRIPTION,  // 3
        GITHUB_URL,         // 4
        CARGO_RUST_VERSION, // 5
    ];

    let info_embed = embeds::info_command_embed(&bot_avatar_url, constants);

    let reply = replies::reply_embed(info_embed, true);
    ctx.send(reply).await?;

    Ok(())
}
