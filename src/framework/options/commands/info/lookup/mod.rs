// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod server;

use crate::{framework::options::commands::info::lookup::server::server, Context, Error};

#[poise::command(
    slash_command,
    subcommands("server"),
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    subcommand_required,
    user_cooldown = 5,
    ephemeral
)]
pub(super) async fn lookup(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
