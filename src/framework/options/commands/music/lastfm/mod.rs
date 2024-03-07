// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod get;
mod search;

use crate::{framework::options::commands::music::lastfm::search::search, Context, Error};

const LASTFM_COLOUR: u32 = 0xA90000;

#[poise::command(
    slash_command,
    subcommands("search"),
    category = "Music",
    required_bot_permissions = "SEND_MESSAGES",
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn lastfm(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
