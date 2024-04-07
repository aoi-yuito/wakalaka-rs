// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::Framework;
use wakalaka_core::{options, types::Error, Data};

pub(super) async fn build_framework(data: Data) -> Framework<Data, Error> {
    Framework::builder()
        .setup(|ctx, ready, framework| {
            Box::pin(wakalaka_core::fetch_user_data(ctx, ready, framework, data))
        })
        .options(options::fetch_framework_options().await)
        .build()
}
