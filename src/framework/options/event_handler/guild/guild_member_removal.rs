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

use serenity::all::{GuildId, Member, User};
use tracing::info;

use crate::{serenity::Context, utility::models};

pub async fn handle(guild_id: &GuildId, user: &User, member: &Option<Member>, ctx: &Context) {
    if let Some(member) = member {
        let guild_id = member.guild_id;
        let guild_name = models::guilds::guild_name_from_guild_id_raw(ctx, guild_id).await;

        let member_name = &member.user.name;

        info!("@{member_name} left from {guild_name}")
    } else {
        let guild_name = models::guilds::guild_name_from_guild_id_raw(ctx, *guild_id).await;

        let user_name = &user.name;

        info!("@{user_name} left from {guild_name}")
    }
}
