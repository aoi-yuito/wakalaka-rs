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

use std::time::Duration;

use tracing::error;

use crate::{utility::components::messages, Context};

pub(crate) async fn handle(remaining_cooldown: Duration, ctx: Context<'_>) {
    let remaining_seconds = remaining_cooldown.as_secs();

    let reply = if remaining_seconds == 1 {
        messages::warn_reply(
            "You're too fast! Please wait `1` second before trying again.",
            true,
        )
    } else {
        messages::warn_reply(
            format!(
                "You're too fast! Please wait `{remaining_seconds}` seconds before trying again."
            ),
            true,
        )
    };
    if let Err(why) = ctx.send(reply).await {
        error!("Failed to send reply: {why:?}");
    }
}
