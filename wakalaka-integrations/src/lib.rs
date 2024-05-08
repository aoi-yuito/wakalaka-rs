// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod kawaii;

use reqwest::{header, Client};
use wakalaka_core::types::{JsonValue, Throwable};

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub(crate) async fn fetch_json_from_get_request(endpoint: &str) -> Throwable<JsonValue> {
    let params = kawaii::build_parameters()?;

    let client = Client::new();

    let response = client
        .get(format!("{}{endpoint}", kawaii::API_BASE_URL))
        .query(&params)
        .header(header::USER_AGENT, APP_USER_AGENT)
        .header(header::ACCEPT, "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to send GET request to {endpoint}: {e:?}"))?;

    let json = response
        .json::<JsonValue>()
        .await
        .map_err(|e| format!("Failed to parse JSON from response: {e:?}"))?;
    Ok(json)
}
