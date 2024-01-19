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

use serenity::all::{GuildId, User, UserId};
use tracing::{error, warn};

use crate::Context;

pub(super) mod buttons;
pub(super) mod embeds;
pub(super) mod messages;

pub(crate) fn owner_id(ctx: Context<'_>) -> UserId {
    let guild = match ctx.guild() {
        Some(value) => value,
        None => {
            warn!("Couldn't get guild");
            return UserId::new(0);
        }
    };
    guild.owner_id
}

pub(crate) fn guild_name(ctx: Context<'_>) -> String {
    let guild = match ctx.guild() {
        Some(value) => value,
        None => {
            warn!("Couldn't get guild");
            return String::new();
        }
    };
    guild.name.clone()
}

pub(crate) fn guild_id(ctx: Context<'_>) -> GuildId {
    let guild = match ctx.guild() {
        Some(value) => value,
        None => {
            warn!("Couldn't get guild");
            return GuildId::new(0);
        }
    };
    guild.id
}

pub(crate) async fn user(user_id: UserId, ctx: Context<'_>) -> User {
    match user_id.to_user(&ctx).await {
        Ok(user) => user,
        Err(why) => {
            error!("Couldn't get user: {why:?}");
            return User::default();
        }
    }
}
