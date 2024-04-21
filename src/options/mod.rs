// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod command_check;
mod on_error;

use poise::{FrameworkOptions, PrefixFrameworkOptions};
use wakalaka_core::{types::Error, Data};

use crate::{commands, events};

pub(super) async fn fetch_framework_options() -> FrameworkOptions<Data, Error> {
    FrameworkOptions {
        commands: commands::gather_all_commands().await,
        on_error: |error| Box::pin(on_error::handle_on_error_option(error)),
        command_check: Some(|ctx| Box::pin(command_check::handle_command_check_option(ctx))),
        event_handler: |ctx, event, framework_ctx, data| {
            Box::pin(events::handle_event_handler_option(
                ctx,
                event,
                framework_ctx,
                data,
            ))
        },
        prefix_options: fetch_prefix_framework_options().await,
        ..Default::default()
    }
}

async fn fetch_prefix_framework_options() -> PrefixFrameworkOptions<Data, Error> {
    PrefixFrameworkOptions {
        prefix: Some(format!("::")),
        mention_as_prefix: false,
        ignore_edits_if_not_yet_responded: true,
        execute_self_messages: false,
        case_insensitive_commands: true,
        ..Default::default()
    }
}
