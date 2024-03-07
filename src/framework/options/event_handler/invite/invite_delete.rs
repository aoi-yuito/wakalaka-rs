// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::InviteDeleteEvent;
use tracing::{info, warn};

use crate::{utils::models, Error, SContext};

pub(crate) async fn handle(ctx: &SContext, create_evt: &InviteDeleteEvent) -> Result<(), Error> {
    let channel_id = &create_evt.channel_id;
    let channel = channel_id.to_channel(ctx).await?;

    let guild_id = match create_evt.guild_id {
        Some(guild_id) => guild_id,
        None => {
            warn!("No server found for {channel_id}");
            return Ok(());
        }
    };
    let guild = models::guilds::guild_from_id_raw(ctx, &guild_id)?;
    let guild_name = &guild.name;

    let guild_channel = match channel.guild() {
        Some(guild_channel) => guild_channel,
        None => {
            warn!("No channel found in {guild_name}");
            return Ok(());
        }
    };
    let guild_channel_name = &guild_channel.name;

    let code = &create_evt.code;

    info!("Deleted invite to #{guild_channel_name} in {guild_name}: {code}");

    Ok(())
}
