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

use serenity::all::GuildId;
use tracing::info;

use crate::{
    database::{guilds, restricted_guilds},
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    owners_only,
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Allow inviting yours truly to a server.
pub async fn server(
    ctx: Context<'_>,
    #[description = "The server to allow invitation to."]
    #[rename = "server"]
    other_guild_id: GuildId,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let other_guild_name = models::guilds::guild_name(ctx, other_guild_id);

    let failsafe_query = guilds::select_guild_id_from_guilds(&other_guild_id, &pool).await;
    let result = match failsafe_query {
        Some(guild_id) if guild_id == other_guild_id => Err(format!(
            "{other_guild_name} is already allowed to invite yours truly."
        )),
        _ => {
            let previous_query =
                restricted_guilds::select_guild_id_from_restricted_guilds(&other_guild_id, &pool)
                    .await;
            match previous_query {
                Ok(_) => {
                    info!("Allowed {other_guild_name} to invite yours truly");
                    restricted_guilds::delete_from_restricted_guilds(&other_guild_id, pool).await?;
                    Ok(format!(
                        "Yours truly can now be invited to {other_guild_name}."
                    ))
                }
                _ => Err(format!(
                    "Invitation to {other_guild_name} is already allowed!"
                )),
            }
        }
    };

    let reply = match result {
        Ok(message) => messages::ok_reply(None, message, true),
        Err(message) => messages::error_reply(None, message, true),
    };
    ctx.send(reply).await?;

    Ok(())
}
