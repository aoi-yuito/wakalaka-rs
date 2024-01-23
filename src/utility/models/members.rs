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

use serenity::all::{GuildId, Member, UserId};
use tracing::error;

use crate::Context;

pub async fn member(ctx: Context<'_>, guild_id: GuildId, user_id: UserId) -> Member {
    match guild_id.member(&ctx, user_id).await {
        Ok(member) => member,
        Err(why) => {
            error!("Couldn't get member: {why:?}");
            return Member::default();
        }
    }
}

pub async fn members_raw(ctx: &crate::serenity::Context, guild_id: &GuildId) -> Vec<Member> {
    match guild_id.members(&ctx, None, None).await {
        Ok(users) => users,
        Err(why) => {
            error!("Couldn't get members: {why:?}");
            return Vec::new();
        }
    }
}

pub async fn members(ctx: Context<'_>, guild_id: GuildId) -> Vec<Member> {
    match guild_id.members(&ctx, None, None).await {
        Ok(users) => users,
        Err(why) => {
            error!("Couldn't get members: {why:?}");
            return Vec::new();
        }
    }
}
