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

use serenity::all::CommandInteraction;

use crate::Context;
pub mod core;

use tracing::{log::error, log::warn};

pub async fn has_administrator_permission(ctx: &Context, interaction: &CommandInteraction) -> bool {
    let guild_id = match interaction.guild_id {
        Some(guild_id) => guild_id,
        None => return false,
    };

    let member = guild_id
        .member(&ctx.http, interaction.user.id)
        .await
        .unwrap_or_else(|why| {
            error!("{why}");
            panic!("Error while retrieving guild member");
        });

    let cache = &ctx.cache;

    let permissions = member.permissions(cache);
    if let Ok(permissions) = permissions {
        return permissions.administrator();
    }

    let user_name = &interaction.user.name;
    let command_name = &interaction.data.name;
    let channel_name = &interaction
        .channel_id
        .name(&ctx)
        .await
        .unwrap_or_else(|why| {
            error!("{why}");
            panic!("Error while retrieving channel name");
        });
    warn!("@{user_name} doesn't have permission(s) to execute {command_name:?} in #{channel_name}");

    return false;
}
