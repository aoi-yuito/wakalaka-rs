// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod add_tags;
mod get_correction;
mod get_info;
mod get_similar;
mod get_tags;
mod get_top_albums;
mod get_top_tags;
mod get_top_tracks;
mod remove_tag;
mod search;

use crate::{
    framework::options::commands::music::lastfm::artist::{
        add_tags::addtags, get_correction::getcorrection, get_info::getinfo,
        get_similar::getsimilar, get_tags::gettags, get_top_albums::gettopalbums,
        get_top_tags::gettoptags, get_top_tracks::gettoptracks, remove_tag::removetag,
        search::search,
    },
    Context, Throwable,
};

#[poise::command(
    slash_command,
    subcommands(
        "addtags",
        "getcorrection",
        "getinfo",
        "getsimilar",
        "gettags",
        "gettopalbums",
        "gettoptags",
        "gettoptracks",
        "removetag",
        "search"
    ),
    category = "Music",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn artist(_ctx: Context<'_>) -> Throwable<()> {
    Ok(())
}
