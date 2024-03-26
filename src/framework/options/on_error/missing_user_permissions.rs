// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::model::Permissions;
use tracing::error;

use crate::{utils::builders, Context};

pub(crate) async fn handle(permissions: Option<Permissions>, ctx: Context<'_>) {
    let separated_permissions = permissions
        .iter()
        .map(|permissions| format!("{permissions}"))
        .collect::<Vec<_>>()
        .join(", ");

    let reply = builders::replies::error_reply_embed(
        format!("Missing the following permissions: {separated_permissions}"),
        true,
    );

    if let Err(why) = ctx.send(reply).await {
        error!("Failed to send reply: {why}");
    }
}
