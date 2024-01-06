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
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

pub mod core;
pub mod general;
pub mod moderation;
pub mod web;

use crate::Context;
use serenity::all::{ CommandDataOption, CommandInteraction, ChannelId, UserId };
use tracing::{ log::error, log::warn };

pub(crate) async fn is_testing_channel(ctx: &Context, interaction: &CommandInteraction) -> bool {
    let channel_name = &interaction.channel_id.name(&ctx).await.unwrap_or_else(|why| {
        error!("Error while retrieving channel name: {why}");
        panic!("{why:?}");
    });

    if channel_name == "lurid-bot-testing" {
        return true;
    }

    let user_name = &interaction.user.name;
    let command_name = &interaction.data.name;
    warn!(
        "@{user_name} tried to execute {command_name:?} in #{channel_name} but it's not testing channel"
    );

    return false;
}

fn command(interaction: &CommandInteraction, index: usize) -> &CommandDataOption {
    let command = interaction.data.options.get(index).expect("Error while getting command");
    command
}

async fn is_administrator(ctx: &Context, interaction: &CommandInteraction) -> bool {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => {
            return false;
        }
    };

    let member = guild_id.member(&ctx.http, interaction.user.id).await.unwrap_or_else(|why| {
        error!("Error while retrieving guild member: {why}");
        panic!("{why:?}");
    });

    let permissions = member.permissions(&ctx.cache);
    if let Ok(permissions) = permissions {
        return permissions.administrator();
    }

    let user_name = &interaction.user.name;
    let command_name = &interaction.data.name;
    let channel_name = &interaction.channel_id.name(&ctx).await.unwrap_or_else(|why| {
        error!("Error while retrieving channel name: {why}");
        panic!("{why:?}");
    });
    warn!("@{user_name} doesn't have permission(s) to execute {command_name:?} in #{channel_name}");

    return false;
}
