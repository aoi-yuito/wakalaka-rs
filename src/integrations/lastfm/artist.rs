// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;

use crate::{integrations, Throwable};

use super::Method;

pub(crate) async fn add_tags(
    artist: impl Into<String>,
    tags: impl Into<String>,
    sk: impl Into<String>,
) -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::ArtistAddTags.into());
    payload.insert("artist", artist.into());
    payload.insert("tags", tags.into());
    payload.insert("sk", sk.into());

    integrations::lastfm::root_post_json(payload).await
}

pub(crate) async fn get_correction(artist: impl Into<String>) -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::ArtistGetCorrection.into());
    payload.insert("artist", artist.into());

    integrations::lastfm::root_get_json(payload).await
}

pub(crate) async fn get_info(
    artist: impl Into<String>,
    mbid: Option<impl Into<String>>,
    lang: Option<impl Into<String>>,
    autocorrect: Option<bool>,
    username: impl Into<String>,
) -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::ArtistGetInfo.into());
    payload.insert("artist", artist.into());

    if let Some(mbid) = mbid {
        payload.insert("mbid", mbid.into());
    }
    if let Some(lang) = lang {
        payload.insert("lang", lang.into());
    }
    if let Some(autocorrect) = autocorrect {
        payload.insert("autocorrect".into(), format!("{}", autocorrect as u8));
    }

    payload.insert("username", username.into());

    integrations::lastfm::root_get_json(payload).await
}

pub(crate) async fn get_similar(
    artist: impl Into<String>,
    limit: Option<u8>,
    autocorrect: Option<bool>,
    mbid: Option<impl Into<String>>,
) -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::ArtistGetSimilar.into());
    payload.insert("artist", artist.into());

    if let Some(limit) = limit {
        payload.insert("limit", format!("{limit}"));
    }
    if let Some(autocorrect) = autocorrect {
        payload.insert("autocorrect", format!("{}", autocorrect as u8));
    }
    if let Some(mbid) = mbid {
        payload.insert("mbid", mbid.into());
    }

    integrations::lastfm::root_get_json(payload).await
}

pub(crate) async fn get_tags(
    artist: impl Into<String>,
    mbid: Option<impl Into<String>>,
    user: impl Into<String>,
    autocorrect: Option<bool>,
) -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::ArtistGetTags.into());
    payload.insert("artist", artist.into());

    if let Some(mbid) = mbid {
        payload.insert("mbid", mbid.into());
    }

    payload.insert("user", user.into());

    if let Some(autocorrect) = autocorrect {
        payload.insert("autocorrect", format!("{}", autocorrect as u8));
    }

    integrations::lastfm::root_get_json(payload).await
}

pub(crate) async fn get_top_albums(
    artist: impl Into<String>,
    mbid: Option<impl Into<String>>,
    autocorrect: Option<bool>,
    page: Option<u8>,
    limit: Option<u8>,
) -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::ArtistGetTopAlbums.into());
    payload.insert("artist", artist.into());

    if let Some(mbid) = mbid {
        payload.insert("mbid", mbid.into());
    }
    if let Some(autocorrect) = autocorrect {
        payload.insert("autocorrect", format!("{}", autocorrect as u8));
    }
    if let Some(page) = page {
        payload.insert("page", format!("{page}"));
    }
    if let Some(limit) = limit {
        payload.insert("limit", format!("{limit}"));
    }

    integrations::lastfm::root_get_json(payload).await
}

pub(crate) async fn get_top_tags(
    artist: impl Into<String>,
    mbid: Option<impl Into<String>>,
    autocorrect: Option<bool>,
) -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::ArtistGetTopTags.into());
    payload.insert("artist", artist.into());

    if let Some(mbid) = mbid {
        payload.insert("mbid", mbid.into());
    }
    if let Some(autocorrect) = autocorrect {
        payload.insert("autocorrect", format!("{}", autocorrect as u8));
    }

    integrations::lastfm::root_get_json(payload).await
}

pub(crate) async fn get_top_tracks(
    artist: impl Into<String>,
    mbid: Option<impl Into<String>>,
    autocorrect: Option<bool>,
    page: Option<u8>,
    limit: Option<u8>,
) -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::ArtistGetTopTracks.into());
    payload.insert("artist", artist.into());

    if let Some(mbid) = mbid {
        payload.insert("mbid", mbid.into());
    }
    if let Some(autocorrect) = autocorrect {
        payload.insert("autocorrect", format!("{}", autocorrect as u8));
    }
    if let Some(page) = page {
        payload.insert("page", format!("{page}"));
    }
    if let Some(limit) = limit {
        payload.insert("limit", format!("{limit}"));
    }

    integrations::lastfm::root_get_json(payload).await
}

pub(crate) async fn remove_tag(
    artist: impl Into<String>,
    tag: impl Into<String>,
    sk: impl Into<String>,
) -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::ArtistRemoveTag.into());
    payload.insert("artist", artist.into());
    payload.insert("tag", tag.into());
    payload.insert("sk", sk.into());

    integrations::lastfm::root_post_json(payload).await
}

pub(crate) async fn search(
    artist: impl Into<String>,
    limit: Option<u8>,
    page: Option<u8>,
) -> Throwable<serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("method", Method::ArtistSearch.into());
    payload.insert("artist", artist.into());

    if let Some(limit) = limit {
        payload.insert("limit", format!("{limit}"));
    }
    if let Some(page) = page {
        payload.insert("page", format!("{page}"));
    }

    integrations::lastfm::root_get_json(payload).await
}
