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
    utility::{components::messages, models},
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
/// Remove a specific warning from a user.
pub(crate) async fn unwarn(
    ctx: Context<'_>,
    #[description = "The user to unwarn."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "ID of the warning to delete."]
    #[rename = "id"]
    #[min = 1]
    case_id: i32,
    #[description = "The reason for unwarning, if any."]
    #[min_length = 6]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user = models::users::user(ctx, user_id).await;
    if user.bot || user.system {
        let reply =
            messages::error_reply("Sorry, but bots and system users cannot be unwarned.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    if case_id < 1 {
        let reply = messages::warn_reply("I'm afraid case ID must be greater than `0`.", true);
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

    let warn_type = InfractionType::Warn.as_str();

    let infractions = infractions::infractions(user_id, guild_id, warn_type, pool).await?;

    let number_of_infractions = infractions.len();
    if number_of_infractions < 1 {
        let reply = messages::warn_reply(
            format!("I'm afraid <@{user_id}> hasn't been punished before."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    for infraction in infractions {
        let case_id = infraction.0;
        if case_id != case_id {
            let reply = messages::error_reply(
                format!("Sorry, but I couldn't find a warning for <@{user_id}>."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            break;
        }

        let mut user_infractions = match users::infractions(user_id, guild_id, pool).await {
            Some(infractions) => infractions,
            None => {
                warn!("Couldn't get infractions for @{user_name} in database");
                return Ok(());
            }
        };

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

        infractions::delete_infraction(case_id, warn_type, pool).await;

        if let Some(reason) = reason.clone() {
            let number_of_reason = reason.chars().count();
            if number_of_reason < 6 || number_of_reason > 80 {
                let reply = messages::warn_reply(
                    "I'm afraid the reason has to be between `6` and `80` characters.",
                    true,
                );
                if let Err(why) = ctx.send(reply).await {
                    error!("Couldn't send reply: {why:?}");
                    return Err(why.into());
                }

                return Ok(());
            }

            info!("@{user_name} unwarned by @{moderator_name}: {reason}");
        } else {
            info!("@{user_name} unwarned by @{moderator_name}");
        }

        let reply = messages::ok_reply(format!("I've removed a warning from <@{user_id}>."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        break;
    }

    Ok(())
}
