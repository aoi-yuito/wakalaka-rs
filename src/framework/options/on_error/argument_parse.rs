// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::error;

use crate::{utils::components, Context, Error};

pub(crate) async fn handle(error: Error, input: Option<String>, ctx: Context<'_>) {
    let command = ctx.command();
    let command_name = &command.qualified_name;

    let result = if let Some(input) = input {
        if input == ctx.prefix() {
            return;
        }

        error!("Failed to parse arguments for {command_name:?}: {error:?}");

        if input.len() >= 18 && input.len() <= 19 {
            Ok(format!("{input:?} could not be found!"))
        } else {
            Ok(format!(
                "{input:?} is not a valid argument for {command_name}`!"
            ))
        }
    } else {
        error!("Failed to parse arguments for {command_name:?}: {error:?}");
        Err(format!(
            "An error occurred while parsing arguments for `{command_name}`."
        ))
    };

    let reply = match result {
        Ok(message) => components::replies::error_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    if let Err(why) = ctx.send(reply).await {
        error!("Failed to send reply: {why:?}");
    }
}
