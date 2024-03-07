// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::GuildId;
use tracing::info;

use crate::Error;

pub(crate) async fn handle(guild_ids: &Vec<GuildId>) -> Result<(), Error> {
    let guild_id_count = guild_ids.len();
    if guild_id_count == 1 {
        info!("Readied cache for 1 server");
    } else {
        info!("Readied cache for {guild_id_count} servers");
    }

    Ok(())
}
