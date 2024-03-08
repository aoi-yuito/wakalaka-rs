// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{database, Context, Throwable};

pub(crate) async fn handle(ctx: Context<'_>) -> Throwable<bool> {
    let db = &ctx.data().db;

    let author = ctx.author();

    let user_restricted = database::checks::check_restricted_user(ctx, db, author).await?;
    if user_restricted {
        Ok(false)
    } else {
        Ok(true)
    }
}
