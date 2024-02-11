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

use serenity::all::{Mentionable, UserId};

use crate::{Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Fun",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Comfort one of your pals.
pub async fn hug(
    ctx: Context<'_>,
    #[description = "The user to comfort."]
    #[rename = "user"]
    user_id: UserId,
) -> Result<(), Error> {
    let user = user_id.to_user(ctx).await?;

    let user_mention = ctx.author().mention();
    let other_mention = user.mention();

    let message = format!("{user_mention} 🫂 {other_mention}");
    ctx.say(message).await?;

    Ok(())
}
