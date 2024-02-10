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
        components::{embeds, replies},
        models,
    },
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
    let bot = models::users::bot(ctx).await?;
    let bot_face = bot.face();

    let constants = [
        CARGO_NAME,         // 0
        CARGO_VERSION,      // 1
        CARGO_AUTHORS,      // 2
        CARGO_DESCRIPTION,  // 3
        GITHUB_URL,         // 4
        CARGO_RUST_VERSION, // 5
    ];

    let info_embed = embeds::info_command_embed(&bot_face, constants);

    let reply = replies::reply_embed(info_embed, true);
    ctx.send(reply).await?;

    Ok(())
}
