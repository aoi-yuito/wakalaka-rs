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

use serenity::{
    all::{Role, RoleId},
    model::ModelError,
};
use tracing::error;

use crate::Context;

use super::guilds;

pub fn role_name(role: &Role) -> &String {
    &role.name
}

pub async fn role_ids(roles: Vec<Role>) -> Vec<RoleId> {
    roles.iter().map(|role| role.id).collect::<Vec<_>>()
}

pub fn roles(ctx: Context<'_>) -> Result<Vec<Role>, ModelError> {
    let guild = guilds::guild(ctx)?;

    let roles = guild
        .roles
        .into_iter()
        .map(|(_, role)| role)
        .collect::<Vec<_>>();

    let role_count = roles.len();
    if role_count != 0 {
        Ok(roles)
    } else {
        error!("Couldn't get roles");
        Err(ModelError::RoleNotFound)
    }
}
