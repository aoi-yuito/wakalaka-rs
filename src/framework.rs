// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::Framework;
use wakalaka_core::{types::Error, Data};

use crate::options;

pub(super) async fn build_framework(data: Data) -> Framework<Data, Error> {
    Framework::builder()
        .setup(|_, _, _| Box::pin(wakalaka_core::fetch_user_data(data)))
        .options(options::fetch_framework_options().await)
        .build()
}
