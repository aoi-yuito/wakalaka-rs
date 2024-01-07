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

use serenity::all::Command;
use serenity::all::GuildId;
use tracing::log::info;

use crate::Context;
use crate::events;

pub async fn handle(ctx: Context, guilds: Vec<GuildId>) {
    let guild_count = guilds.len();
    info!("Prepared cache for {guild_count} guild(s)");

    let guild_ids = &ctx.cache.guilds();
    for guild_id in guild_ids {
        let guild_name = {
            let guild = &ctx.cache
                .guild(guild_id)
                .expect("Expected guild in cache, but didn't find one");
            guild.name.clone()
        };
        info!("Connected to {guild_name}");

        let (existing_guild_commands, existing_global_commands) = (
            guild_id
                .get_commands(&ctx.http).await
                .expect("Expected existing guild commands, but didn't find any"),
            Command::get_global_commands(&ctx.http),
        );

        events::register_commands(
            &ctx,
            guild_id,
            &guild_name,
            existing_guild_commands,
            existing_global_commands.await.expect(
                "Expected existing global commands, but didn't find any"
            )
        ).await;
    }
}
