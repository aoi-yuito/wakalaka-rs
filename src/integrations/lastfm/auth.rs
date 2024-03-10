// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;

use crate::{
    integrations,
    utils::{self, environment},
    Throwable,
};

use super::Method;

pub(crate) async fn get_session(token: String) -> Throwable<serde_json::Value> {
    let get_session: String = Method::AuthGetSession.into();

    let key = environment::lastfm_api_key()?;
    let secret = environment::lastfm_api_secret()?;

    let api_sig = utils::md5(format!(
        "api_key{key}method{get_session}token{token}{secret}"
    ));

    let mut payload = HashMap::new();
    payload.insert("method", get_session);
    payload.insert("token", token);
    payload.insert("api_sig", api_sig);

    integrations::lastfm::root_get_json(payload).await
}

pub(crate) async fn get_token() -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::AuthGetToken.into());

    integrations::lastfm::root_get_json(payload).await
}
