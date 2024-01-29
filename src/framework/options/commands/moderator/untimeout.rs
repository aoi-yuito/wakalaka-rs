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
    check_restricted_guild_channel,
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
    #[min_length = 6]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let pool = &ctx.data().pool;

    let user = models::users::user(ctx, user_id).await;
    if user.bot || user.system {
        let reply = messages::error_reply(
            "Sorry, but bots and system users cannot be timed out.",
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let user_name = &user.name;

    let moderator = ctx.author();
    let moderator_name = &moderator.name;

    let guild_id = models::guilds::guild_id(ctx).await;

    let mut user_infractions = users::select_infractions_from_users(&user_id, pool).await?;
    if user_infractions < 1 {
        let reply =
            messages::info_reply(format!("<@{user_id}> hasn't been punished before."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let timeouts =
        infractions::select_from_infractions(InfractionType::Timeout, &user_id, &guild_id, pool)
            .await?;
    for timeout in timeouts {
        let uuid = timeout.0;

        let mut member = models::members::member(ctx, guild_id, user_id).await;

        if let Err(why) = member.enable_communication(ctx).await {
            error!("Couldn't get member out of time-out: {why:?}");

            let reply = messages::error_reply(
                format!("Sorry, but I couldn't get <@{user_id} out of a time-out."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        }

        guild_members::update_guilds_members_set_timeout(&user_id, false, None, pool).await?;

        if let Some(ref reason) = reason {
            let reason_char_count = reason.chars().count();
            if reason_char_count < 6 || reason_char_count > 80 {
                let reply = messages::info_reply(
                    "Reason must be between `6` and `80` characters long.",
                    true,
                );
                if let Err(why) = ctx.send(reply).await {
                    error!("Couldn't send reply: {why:?}");
                    return Err(why.into());
                }

                return Ok(());
            }

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

        let reply = messages::ok_reply(
            format!("<@{user_id}> has been released from a time-out."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }
    }

    Ok(())
}
