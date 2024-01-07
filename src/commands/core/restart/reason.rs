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

use serenity::all::ResolvedValue;
use serenity::model::application::ResolvedOption;
use tokio::time::Duration;
use tracing::log::info;

use crate::Context;

pub(super) fn reason(ctx: &Context, options: &[ResolvedOption<'_>]) -> Option<String> {
    let reason = options
        .get(0)
        .and_then(|option| {
            match &option.value {
                ResolvedValue::String(s) => Some(s),
                _ => None,
            }
        })
        .expect("Error while getting reason");

    let reason_characters_count = reason.chars().count();
    if reason_characters_count > 50 {
        return Some(format!("Reason cannot be more than 50 characters."));
    } else if reason_characters_count < 3 {
        return Some(format!("Reason cannot be less than 3 characters."));
    }

    let seconds = 5u64;
    info!("Restarting in {seconds} seconds: {reason}");

    let cloned_ctx = ctx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(seconds)).await;

        cloned_ctx.shard.shutdown_clean();
    });

    Some(format!("Restarting in {seconds} seconds..."))
}
