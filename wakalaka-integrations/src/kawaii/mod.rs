// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod gif;

use std::collections::HashMap;

use wakalaka_core::{envs, types::Throwable};

pub const KAWAII_LOGO_URL: &str = "https://kawaii.red/assets/img/logo-small.webp";

pub const API_BASE_URL: &str = "https://kawaii.red/api/";

pub(super) fn build_parameters() -> Throwable<HashMap<&'static str, String>> {
    let token = envs::fetch_api_kawaii_token_from_env()?;

    let mut params = HashMap::new();
    params.insert("token", token);
    Ok(params)
}
