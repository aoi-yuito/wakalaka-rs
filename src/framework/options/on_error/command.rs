// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::error;

use crate::{utils::components, Context, Error};

pub(crate) async fn handle(error: Error, ctx: Context<'_>) {
    let command = ctx.command();
    let command_name = &command.qualified_name;

    error!("Failed to invoke {command_name:?}: {error:?}");

    let reply = components::replies::error_reply_embed(
        format!("An error occurred while invoking `{command_name}`."),
        true,
    );

    if let Err(why) = ctx.send(reply).await {
        error!("Failed to send reply: {why:?}");
    }
}
