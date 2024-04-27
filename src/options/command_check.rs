// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use wakalaka_core::types::{Context, Throwable};

use wakalaka_database::checks;

pub(super) async fn handle_command_check_option(ctx: Context<'_>) -> Throwable<bool> {
    let db = &ctx.data().db;

    let user = ctx.author();
    let user_id = &user.id;

    let user_restricted = checks::is_user_restricted(db, ctx, user_id).await?;
    if user_restricted {
        Ok(false)
    } else {
        Ok(true)
    }
}
