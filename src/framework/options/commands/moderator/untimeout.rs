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
use tracing::{error, info, warn};

use crate::{
    database::{
        infractions::{self, InfractionType},
        users,
    },
    utility::{self, components::messages},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    guild_only,
    ephemeral
)]
/// Get a user out of a time-out.
pub(crate) async fn untimeout(
    ctx: Context<'_>,
    #[description = "The user to get out of a time-out from."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for getting out of a time-out, if any. (6-80)"]
    #[min_length = 6]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user = utility::users::user(ctx, user_id).await;
    if user.bot || user.system {
        let reply =
            messages::error_reply("Cannot get bots and system users out of a time-out.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let user_name = &user.name;

    let moderator = ctx.author();
    let moderator_name = &moderator.name;

    let guild_id = utility::guilds::guild_id(ctx).await;

    let timeout_type = InfractionType::Timeout.as_str();

    let infractions = infractions::infractions(user_id, guild_id, timeout_type, pool).await?;

    let number_of_infractions = infractions.len();
    if number_of_infractions < 1 {
        let reply =
            messages::warn_reply(format!("<@{user_id}> hasn't been punished before."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    for infraction in infractions {
        let case_id = infraction.0;

        let mut user_infractions = match users::infractions(user_id, guild_id, pool).await {
            Some(infractions) => infractions,
            None => {
                warn!("Couldn't get infractions for @{user_name} in database");
                return Ok(());
            }
        };

        let mut member = utility::guilds::member(ctx, guild_id, user_id).await;
        if let Err(why) = member.enable_communication(ctx).await {
            error!("Couldn't get member out of time-out: {why:?}");

            let reply = messages::error_reply("Couldn't get member out of a time-out.", true);
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        } else {
            user_infractions -= 1;
            if user_infractions < 0 {
                user_infractions = 0;
            }

            users::update_user(
                user_id,
                guild_id,
                user_infractions,
                false,
                false,
                false,
                false,
                pool,
            )
            .await;

            infractions::delete_infraction(case_id, timeout_type, pool).await;

            if let Some(reason) = reason.clone() {
                let number_of_reason = reason.chars().count();
                if number_of_reason < 6 || number_of_reason > 80 {
                    let reply =
                        messages::warn_reply("Reason must be between 8 and 80 characters.", true);
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

            let reply = messages::ok_reply(
                format!("<@{user_id}> has been gotten out of a time-out."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }
        }
    }

    Ok(())
}
