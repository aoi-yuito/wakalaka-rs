// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::time::Duration;

use tracing::error;

use crate::{utils::builders, Context};

pub(crate) async fn handle(cooldown: Duration, ctx: Context<'_>) {
    let remaining_seconds = cooldown.as_secs();

    let reply = if remaining_seconds == 1 || remaining_seconds == 0 {
        builders::replies::warn_reply_embed(
            "Too fast! Wait a second before trying again, okay?",
            true,
        )
    } else {
        builders::replies::warn_reply_embed(
            format!("Too fast! Wait {remaining_seconds} seconds before trying again, okay?"),
            true,
        )
    };

    if let Err(why) = ctx.send(reply).await {
        error!("Failed to send reply: {why:?}");
    }
}
