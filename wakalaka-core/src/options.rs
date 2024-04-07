// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::{types::Error, Data};
use poise::{FrameworkOptions, PrefixFrameworkOptions};

pub async fn fetch_framework_options() -> FrameworkOptions<Data, Error> {
    FrameworkOptions {
        commands: vec![],
        // on_error: |error| Box::pin(options::on_error::handle(error)),
        // post_command: |ctx| Box::pin(options::post_command::handle(ctx)),
        // command_check: Some(|ctx| Box::pin(options::command_check::handle(ctx))),
        // event_handler: |ctx, event, framework_ctx, data| {
        //     Box::pin(options::event_handler::handle(
        //         ctx,
        //         event,
        //         framework_ctx,
        //         data,
        //     ))
        // },
        prefix_options: fetch_prefix_framework_options().await,
        ..Default::default()
    }
}

async fn fetch_prefix_framework_options() -> PrefixFrameworkOptions<Data, Error> {
    PrefixFrameworkOptions {
        prefix: Some(format!("?")),
        mention_as_prefix: false,
        ignore_edits_if_not_yet_responded: true,
        execute_self_messages: false,
        case_insensitive_commands: true,
        ..Default::default()
    }
}
