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

use poise::serenity_prelude::Context;
use serenity::all::GuildId;
use tracing::error;

use crate::Data;

pub(super) async fn handle(guild_ids: &Vec<GuildId>, ctx: &Context, data: &Data) {
    let database = &data.pool;

    let guild = {
        let first_guild_id = guild_ids.first().unwrap();
        match first_guild_id.to_guild_cached(&ctx) {
            Some(guild) => guild.clone(),
            None => {
                error!("Couldn't get first guild from cache");
                return;
            }
        }
    };
    let guild_channels = match guild.channels(&ctx).await {
        Ok(guild_channel) => guild_channel,
        Err(why) => {
            error!("Couldn't get guild channels: {why:?}");
            return;
        }
    };
}
