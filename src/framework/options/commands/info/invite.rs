// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{utils::BOT_INVITE_URL, Context, Throwable};

#[poise::command(
    slash_command,
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5,
    ephemeral
)]
/// Get an invite link for yours truly.
pub(super) async fn invite(ctx: Context<'_>) -> Throwable<()> {
    ctx.say(BOT_INVITE_URL).await?;

    Ok(())
}
