// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod options;

use poise::{Framework, FrameworkOptions};

use crate::{Data, Error, SContext, SReady, Throwable};

pub(crate) async fn framework(data: Data) -> Framework<Data, Error> {
    Framework::builder()
        .setup(|ctx, ready, framework| Box::pin(framework_setup(ctx, ready, framework, data)))
        .options(framework_options().await)
        .build()
}

async fn framework_setup(
    _ctx: &SContext,
    _ready: &SReady,
    _framework: &Framework<Data, Error>,
    data: Data,
) -> Throwable<Data> {
    Ok(data)
}

async fn framework_options() -> FrameworkOptions<Data, Error> {
    FrameworkOptions {
        commands: options::commands::commands().await,
        on_error: |error| Box::pin(options::on_error::handle(error)),
        post_command: |ctx| Box::pin(options::post_command::handle(ctx)),
        command_check: Some(|ctx| Box::pin(options::command_check::handle(ctx))),
        event_handler: |ctx, event, framework_ctx, data| {
            Box::pin(options::event_handler::handle(
                ctx,
                event,
                framework_ctx,
                data,
            ))
        },
        prefix_options: options::prefix_options::prefix_framework_options(),
        ..Default::default()
    }
}
