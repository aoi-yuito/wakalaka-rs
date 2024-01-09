// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.\

mod core;
mod fun;
mod misc;
mod moderator;

use poise::Command;

use crate::{Context, Data, Error};

#[macro_export]
macro_rules! check_channel_restriction {
    ($ctx:expr) => {
        let channel_restricted = crate::commands::is_channel_restricted($ctx).await;
        if channel_restricted {
            let message = "Sorry, but I can't be utilised in this channel.";
            let _ = $ctx.reply(message).await;

            return Ok(());
        }
    };
}

async fn is_channel_restricted(ctx: Context<'_>) -> bool {
    let channel_id = ctx.channel_id();

    let restricted_channels = ctx.data().restricted_channels.read().await;
    restricted_channels.contains(&channel_id)
}

// pub(crate) async fn global_commands() -> Vec<Command<Data, Error>> {
//     vec![]
// }

pub(crate) async fn guild_commands() -> Vec<Command<Data, Error>> {
    vec![
        core::info::info(),
        core::restart::restart(),
        core::restrict::restrict(),
        core::shutdown::shutdown(),
        core::unrestrict::unrestrict(),
        fun::hug::hug(),
        misc::avatar::avatar(),
        misc::suggest::suggest(),
        moderator::purge::purge(),
    ]
}
