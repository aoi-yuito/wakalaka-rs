// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::CurrentUser;
use wakalaka_core::types::{SContext, Throwable};

pub async fn bot_raw(ctx: &SContext) -> Throwable<CurrentUser> {
    let http = &ctx.http;

    let bot = http.get_current_user().await?;
    Ok(bot)
}
