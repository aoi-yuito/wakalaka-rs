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

use serenity::all::UserId;
use tracing::{error, info};

use crate::{
    database::{
        guild_members,
        infractions::{self, InfractionType},
        users,
    },
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "SEND_MESSAGES | MODERATE_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Get a user out of a time-out.
pub async fn untimeout(
    ctx: Context<'_>,
    #[description = "The user to get out of a time-out from."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for getting out of a time-out, if any."]
    #[min_length = 3]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user = models::users::user(ctx, user_id).await?;
    let (user_name, user_mention) = (&user.name, models::users::user_mention(ctx, user_id).await?);

    let moderator = models::users::author(ctx)?;
    let (moderator_id, moderator_name) = (moderator.id, &moderator.name);

    if user.bot || user.system {
        let reply = messages::error_reply(
            "Sorry, but bots and system users cannot be taken out of a time-out.",
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }
    if user_id == moderator_id {
        let reply = messages::error_reply(
            "Sorry, but you cannot get yourself out of a time-out.",
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let guild_id = models::guilds::guild_id(ctx)?;

    let mut user_infractions = users::select_infractions_from_users(&user_id, pool).await?;
    if user_infractions < 1 {
        let reply =
            messages::info_reply(format!("{user_mention} hasn't been punished before."), true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let timeouts =
        infractions::select_from_infractions(InfractionType::Timeout, &user_id, &guild_id, pool)
            .await?;
    for timeout in timeouts {
        let uuid = timeout.0;

        let mut member = models::members::member(ctx, guild_id, user_id).await?;

        if let Err(why) = member.enable_communication(ctx).await {
            error!("Couldn't get member out of time-out: {why:?}");

            let reply = messages::error_reply(
                format!("Sorry, but I couldn't get {user_mention} out of a time-out."),
                true,
            );
            ctx.send(reply).await?;

            return Ok(());
        }

        guild_members::update_guilds_members_set_timeout(&user_id, false, None, pool).await?;

        if let Some(ref reason) = reason {
            info!("@{user_name} got @{moderator_name} out of time-out: {reason}");
        } else {
            info!("@{user_name} got @{moderator_name} out of time-out")
        }

        infractions::delete_from_infractions(&uuid, &guild_id, pool).await?;

        user_infractions -= 1;
        if user_infractions < 0 {
            user_infractions = 0;
        }

        users::update_users_set_infractions(&user_id, user_infractions, pool).await?;

        let reply = messages::ok_reply(format!("Got {user_mention} out of a time-out."), true);
        ctx.send(reply).await?;
    }

    Ok(())
}
