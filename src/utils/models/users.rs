// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::CurrentUser;
use tracing::error;

use crate::{SContext, Throwable};

pub(crate) async fn bot_raw(ctx: &SContext) -> Throwable<CurrentUser> {
    match ctx.http.get_current_user().await {
        Ok(bot) => Ok(bot),
        Err(why) => {
            error!("Failed to get bot: {why:?}");
            Err(why.into())
        }
    }
}
