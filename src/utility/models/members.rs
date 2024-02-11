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
    all::{GuildId, Member},
    model::ModelError,
};
use tracing::error;

use crate::Context;

pub async fn members_raw(
    ctx: &crate::serenity::Context,
    guild_id: &GuildId,
) -> Result<Vec<Member>, ModelError> {
    let mut members = Vec::new();

    let mut after = None;

    loop {
        match guild_id.members(ctx, Some(1000), after).await {
            Ok(mut new_members) => {
                if new_members.is_empty() {
                    return Ok(members);
                }

                after = new_members.last().map(|m| m.user.id);

                members.append(&mut new_members);
            }
            Err(why) => {
                error!("Failed to get members: {why:?}");
                return Err(ModelError::MemberNotFound);
            }
        }
    }
}

pub async fn members(ctx: Context<'_>, guild_id: GuildId) -> Result<Vec<Member>, ModelError> {
    let mut members = Vec::new();

    let mut after = None;

    loop {
        match guild_id.members(ctx, Some(1000), after).await {
            Ok(mut new_members) => {
                if new_members.is_empty() {
                    return Ok(members);
                }

                after = new_members.last().map(|m| m.user.id);

                members.append(&mut new_members);
            }
            Err(why) => {
                error!("Failed to get members: {why:?}");
                return Err(ModelError::MemberNotFound);
            }
        }
    }
}
