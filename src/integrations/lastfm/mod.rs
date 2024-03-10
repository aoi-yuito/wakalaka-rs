// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub(crate) mod artist;
pub(crate) mod auth;

use std::collections::HashMap;

use reqwest::Response;

use crate::{utils::environment, Throwable};

use super::APP_USER_AGENT;

pub(crate) const API_AUTH_URL: &str = "https://www.last.fm/api/auth/";

const API_ROOT_URL: &str = "https://ws.audioscrobbler.com/2.0/";

enum Method {
    ArtistAddTags,
    ArtistGetCorrection,
    ArtistGetInfo,
    ArtistGetSimilar,
    ArtistGetTags,
    ArtistGetTopAlbums,
    ArtistGetTopTags,
    ArtistGetTopTracks,
    ArtistRemoveTag,
    ArtistSearch,
    AuthGetSession,
    AuthGetToken,
}

impl From<Method> for String {
    fn from(method: Method) -> String {
        match method {
            Method::ArtistAddTags => "artist.addTags".into(),
            Method::ArtistGetCorrection => "artist.getCorrection".into(),
            Method::ArtistGetInfo => "artist.getInfo".into(),
            Method::ArtistGetSimilar => "artist.getSimilar".into(),
            Method::ArtistGetTags => "artist.getTags".into(),
            Method::ArtistGetTopAlbums => "artist.getTopAlbums".into(),
            Method::ArtistGetTopTags => "artist.getTopTags".into(),
            Method::ArtistGetTopTracks => "artist.getTopTracks".into(),
            Method::ArtistRemoveTag => "artist.removeTag".into(),
            Method::ArtistSearch => "artist.search".into(),
            Method::AuthGetSession => "auth.getSession".into(),
            Method::AuthGetToken => "auth.getToken".into(),
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}", self))
    }
}

pub(crate) async fn root_post_json(
    mut query: HashMap<&str, String>,
) -> Throwable<serde_json::Value> {
    query.insert("format", "json".into());

    let res_json = root_post_text(query).await?;
    let res_json = serde_json::from_str(&res_json)?;
    Ok(res_json)
}

pub(crate) async fn root_post_text(query: HashMap<&str, String>) -> Throwable<String> {
    let res = root_post(query).await?;
    let res_text = res.text().await?;
    Ok(res_text)
}

pub(crate) async fn root_post(mut query: HashMap<&str, String>) -> Throwable<Response> {
    let key = environment::lastfm_api_key()?;

    query.insert("api_key", key);

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let res = client.post(API_ROOT_URL).form(&query).send().await?;
    Ok(res)
}

pub(crate) async fn get(url: impl Into<String>, token: String) -> Throwable<Response> {
    let key = environment::lastfm_api_key()?;

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let mut query = HashMap::new();
    query.insert("api_key", key);
    query.insert("token", token);

    let res = client.get(url.into()).query(&query).send().await?;
    Ok(res)
}

async fn root_get_json(mut query: HashMap<&str, String>) -> Throwable<serde_json::Value> {
    query.insert("format", "json".into());

    let res_text = root_get_text(query).await?;
    let res_json = serde_json::from_str(&res_text)?;
    Ok(res_json)
}

async fn root_get_text(query: HashMap<&str, String>) -> Throwable<String> {
    let res = root_get(query).await?;
    let res_text = res.text().await?;
    Ok(res_text)
}

async fn root_get(mut query: HashMap<&str, String>) -> Throwable<Response> {
    let key = environment::lastfm_api_key()?;

    query.insert("api_key", key);

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let res = client.get(API_ROOT_URL).query(&query).send().await?;
    Ok(res)
}
