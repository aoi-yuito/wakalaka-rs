/**
 * Copyright (C) 2024 Kasutaja
 *
 * wakalaka-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * wakalaka-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::util::uses::*;

pub async fn setup_framework_options() -> FrameworkOptions<crate::Data, crate::Error> {
    let options = FrameworkOptions {
        commands: vec![crate::booru::aibooru::aibooru()],
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command '{}'...", ctx.command().qualified_name);
            })
        },
        post_command: |_| Box::pin(async move {}),
        event_handler: |ctx, event, framework, data| {
            Box::pin(event::event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };
    options
}

pub async fn build_framework(
    options: FrameworkOptions<crate::Data, crate::Error>,
) -> Framework<crate::Data, crate::Error> {
    let framework = Framework::builder()
        .options(options)
        .setup(|ctx, ready, framework| {
            Box::pin(async move { setup_framework(ctx, ready, framework).await })
        })
        .build();
    framework
}

async fn setup_framework<'a>(
    ctx: &serenity::Context,
    _ready: &Ready,
    framework: &poise::Framework<crate::Data, crate::Error>,
) -> Result<crate::Data, crate::Error> {
    poise::builtins::register_globally(ctx, &framework.options().commands).await?;

    Ok(crate::Data {})
}
