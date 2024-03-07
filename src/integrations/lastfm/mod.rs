// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub(crate) mod artist;

use reqwest::header::USER_AGENT;
use std::collections::HashMap;

use crate::{utils::environment, Error};

use super::POSTMAN_USER_AGENT;

const LASTFM_API_ROOT_URL: &str = "https://ws.audioscrobbler.com/2.0/";

pub(crate) async fn lastfm_get(
    mut payload: HashMap<&'static str, String>,
) -> Result<serde_json::Value, Error> {
    let lastfm_api_key = environment::lastfm_api_key()?;

    payload.insert("api_key", lastfm_api_key);
    payload.insert("format", format!("json"));

    let client = reqwest::Client::new();

    let res = client
        .get(LASTFM_API_ROOT_URL)
        .header(USER_AGENT, POSTMAN_USER_AGENT)
        .query(&payload)
        .send()
        .await?;
    let res_text = res.text().await?;
    let res_json: serde_json::Value = serde_json::from_str(&res_text)?;
    Ok(res_json)
}
