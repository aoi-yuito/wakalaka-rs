// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use wakalaka_core::{
    consts,
    types::{Context, Throwable},
};
use wakalaka_utils::builders;

#[poise::command(
    slash_command,
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5,
    ephemeral
)]
/// Get an invite link for yours truly.
pub async fn invite(ctx: Context<'_>) -> Throwable<()> {
    let reply = builders::replies::build_reply_with_embed(
        format!(
            ":link: [Click here]({}) to invite me to your server.",
            consts::APP_INVITE_URL
        ),
        true,
    );

    ctx.send(reply).await?;

    Ok(())
}
