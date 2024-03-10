// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod lastfm;

use crate::{framework::options::commands::integrations::link::lastfm::lastfm, Context, Throwable};

#[poise::command(
    slash_command,
    subcommands("lastfm"),
    category = "Integrations",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn link(_ctx: Context<'_>) -> Throwable<()> {
    Ok(())
}
