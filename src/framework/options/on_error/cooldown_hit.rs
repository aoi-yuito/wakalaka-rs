// Copyright (c) 2024 Kawaxte
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::time::Duration;

use tracing::error;

use crate::{utils::components, Context};

pub(crate) async fn handle(cooldown: Duration, ctx: Context<'_>) {
    let remaining_seconds = cooldown.as_secs();

    let reply = if remaining_seconds == 1 || remaining_seconds == 0 {
        components::replies::error_reply_embed(
            "You're too fast! Please wait a second before trying again.",
            true,
        )
    } else {
        components::replies::error_reply_embed(
            format!(
                "You're too fast! Please wait {remaining_seconds} seconds before trying again."
            ),
            true,
        )
    };

    if let Err(why) = ctx.send(reply).await {
        error!("Failed to send reply: {why:?}");
    }
}
