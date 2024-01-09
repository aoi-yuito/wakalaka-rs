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

pub(crate) mod settings;

use serenity::all::{Guild, GuildId, Member, UserId};
use tracing::error;

use crate::{serenity::Context, Data, Error};

pub(crate) async fn member(
    guild_id: GuildId,
    ctx: poise::Context<'_, Data, Error>,
    author_id: UserId,
) -> Member {
    match guild_id.member(&ctx.http(), author_id).await {
        Ok(value) => value,
        Err(why) => {
            error!("Couldn't get guild member: {why:?}");
            panic!("{why:?}");
        }
    }
}

pub(crate) fn guild_name_raw(guild_id: &GuildId, ctx: &Context) -> Option<String> {
    let guild_name = {
        let guild = match guild_from_cache(ctx, guild_id) {
            Ok(value) => value,
            Err(value) => return value,
        };
        guild.name.clone()
    };
    Some(guild_name)
}

pub(crate) async fn guild_id_raw(ctx: &Context) -> Option<GuildId> {
    let current_application_info = match ctx.http.get_current_application_info().await {
        Ok(value) => value,
        Err(why) => {
            error!("Couldn't get application info: {why:?}");
            panic!("{why:?}");
        }
    };

    let guild_id = match current_application_info.guild_id {
        Some(value) => value,
        None => return None,
    };
    Some(guild_id)
}

pub(crate) fn guild_from_cache<'a>(
    ctx: &'a Context,
    guild_id: &'a GuildId,
) -> Result<Guild, Option<String>> {
    let guild = {
        let guild = match ctx.cache.guild(*guild_id) {
            Some(value) => value,
            None => return Err(Some("Couldn't get guild from cache".to_string())),
        };
        guild.clone()
    };
    Ok(guild)
}
