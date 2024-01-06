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

use crate::commands::core::restart::delay;
use crate::Context;
use serenity::all::ResolvedValue;
use serenity::model::application::ResolvedOption;
use tokio::time::Duration;
use tracing::log::info;

pub(super) fn reason(ctx: &Context, options: &[ResolvedOption<'_>]) -> Option<String> {
    let reason = options
        .get(0)
        .and_then(|option| {
            match &option.value {
                ResolvedValue::String(s) => Some(s),
                _ => None,
            }
        })
        .unwrap_or(&"Cannot restart if no reason is provided.");
    if reason.len() > 50 {
        return None;
    }

    let seconds = delay
        ::delay(options)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(5);
    info!("Restarting in {seconds} seconds: {reason}");

    let cloned_ctx = ctx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(seconds as u64)).await;

        cloned_ctx.shard.shutdown_clean();
    });

    Some(format!("Restarting in {seconds} seconds..."))
}
