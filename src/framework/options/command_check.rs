// Copyright (c) 2024 Kawaxte
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{database, Context, Error};

pub(crate) async fn handle(ctx: Context<'_>) -> Result<bool, Error> {
    let db = &ctx.data().db;

    let author = ctx.author();

    let user_restricted = database::checks::check_restricted_user(ctx, db, author).await?;
    return if user_restricted { Ok(false) } else { Ok(true) };
}
