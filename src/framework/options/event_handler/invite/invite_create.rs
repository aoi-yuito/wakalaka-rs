// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::InviteCreateEvent;
use tracing::{info, warn};

use crate::{utils::models, SContext, Throwable};

pub(crate) async fn handle(ctx: &SContext, create_evt: &InviteCreateEvent) -> Throwable<()> {
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

    // let inviter = match &create_evt.inviter {
    //     Some(inviter) => inviter,
    //     None => {
    //         warn!("No inviter found for {code}");
    //         return Ok(());
    //     }
    // };

    if let Some(inviter) = &create_evt.inviter {
        info!("@{inviter} created invite to #{guild_channel_name} in {guild_name}: {code}");
    } else {
        info!("Created invite to #{guild_channel_name} in {guild_name}: {code}");
    }

    Ok(())
}
