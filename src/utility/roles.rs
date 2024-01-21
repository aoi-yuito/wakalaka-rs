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

use serenity::all::{Role, RoleId};
use tracing::warn;

use crate::Context;

use super::guilds;

pub(crate) async fn role_ids(roles: Vec<Role>) -> Vec<RoleId> {
    roles.iter().map(|role| role.id).collect::<Vec<RoleId>>()
}

pub(crate) async fn role(ctx: Context<'_>, name: &String) -> Role {
    let guild = guilds::guild(ctx).await;
    let guild_name = &guild.name;

    match guild.role_by_name(&name) {
        Some(role) => role.clone(),
        None => {
            warn!("Couldn't find role named @{name} in {guild_name}");
            return Role::default();
        }
    }
}
