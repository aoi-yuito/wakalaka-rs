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
use tracing::{error, info};

use crate::{
    database::{guilds, restricted_guilds},
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
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

    let other_guild_name = models::guilds::guild_name_from_guild_id(ctx, other_guild_id).await;

    let failsafe_query = guilds::select_guild_id_from_guilds(&other_guild_id, &pool).await;
    if let Some(guild_id) = failsafe_query {
        if guild_id == other_guild_id {
            let reply = messages::warn_reply(
                format!("I've already been allowed to be invited to {other_guild_name}."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        }
    }

    let previous_query =
        restricted_guilds::select_guild_id_from_restricted_guilds(&other_guild_id, &pool).await;
    if let Ok(_) = previous_query {
        info!("Allowed invitation to {other_guild_name}");

        restricted_guilds::delete_from_restricted_guilds(&other_guild_id, pool).await?;

        let reply = messages::ok_reply(
            format!("I've allowed myself to be invited to {other_guild_name}."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let reply = messages::warn_reply(
        format!("I'm already able to be invited to {other_guild_name}."),
        true,
    );
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
