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
pub mod moderation;
pub mod web;
pub mod misc;

use serenity::all::{ CommandDataOption, CommandInteraction };
use tracing::log::warn;

use crate::Context;

fn command_option(interaction: &CommandInteraction, index: usize) -> Option<&CommandDataOption> {
    if let Some(option) = interaction.data.options.get(index) { Some(option) } else { None }
}

async fn has_manage_messages_permission(ctx: &Context, interaction: &CommandInteraction) -> bool {
    let guild_id = interaction.guild_id.expect("Expected guild ID, but didn't find one");

    let current_user_id = interaction.user.id;

    let member = guild_id
        .member(&ctx.http, current_user_id).await
        .expect("Expected guild member, but didn't find one");

    let permissions = member.permissions(&ctx.cache);
    if let Ok(permissions) = permissions {
        return permissions.manage_messages();
    }

    let user_name = &interaction.user.name;
    let command_name = &interaction.data.name;
    let channel_name = &interaction.channel_id
        .name(&ctx).await
        .expect("Expected channel name, but didn't find one");
    warn!("@{user_name} doesn't have permission(s) to execute {command_name:?} in #{channel_name}");

    return false;
}

async fn has_administrator_permission(ctx: &Context, interaction: &CommandInteraction) -> bool {
    let guild_id = interaction.guild_id.expect("Expected guild ID, but didn't find one");

    let current_user_id = interaction.user.id;

    let member = guild_id
        .member(&ctx.http, current_user_id).await
        .expect("Expected guild member, but didn't find one");

    let permissions = member.permissions(&ctx.cache);
    if let Ok(permissions) = permissions {
        return permissions.administrator();
    }

    let user_name = &interaction.user.name;
    let command_name = &interaction.data.name;
    let channel_name = &interaction.channel_id
        .name(&ctx).await
        .expect("Expected channel name, but didn't find one");
    warn!("@{user_name} doesn't have permission(s) to execute {command_name:?} in #{channel_name}");

    return false;
}
