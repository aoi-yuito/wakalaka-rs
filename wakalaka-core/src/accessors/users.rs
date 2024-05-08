// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::types::{SerenityContext, Throwable};
use serenity::all::CurrentUser;

pub async fn fetch_raw_bot_user_info(ctx: &SerenityContext) -> Throwable<CurrentUser> {
    let http = &ctx.http;

    let bot = http.get_current_user().await?;
    Ok(bot)
}
