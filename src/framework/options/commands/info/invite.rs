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

use crate::{framework::options::commands::info::BOT_INVITE_URL, Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Info",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5,
    ephemeral
)]
/// Get an invitation from yours truly.
pub async fn invite(ctx: Context<'_>) -> Result<(), Error> {
    let message = format!("{BOT_INVITE_URL}");
    ctx.say(message).await?;

    Ok(())
}
