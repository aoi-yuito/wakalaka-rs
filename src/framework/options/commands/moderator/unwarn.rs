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

use serenity::all::User;
use tracing::{error, info, warn};

use crate::{
    database::{
        infractions::{self, InfractionType},
        users,
    },
    utility::messages,
    Context, Error,
};

/// Remove a specific warning from a user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    guild_only,
    ephemeral
)]
pub(crate) async fn unwarn(
    ctx: Context<'_>,
    #[description = "The user to unwarn."] user: User,
    #[description = "ID of the warning to delete."] id: i32,
    #[description = "The reason for unwarning. (6-80)"] reason: Option<String>,
) -> Result<(), Error> {
    if user.bot || user.system {
        let reply = messages::error_reply("Can't remove warnings from bots or system users");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    if id < 1 {
        let reply = messages::warn_reply("Case ID must be greater than 0");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let pool = &ctx.data().pool;

    let user_id = user.id;
    let user_name = &user.name;

    let moderator = ctx.author();
    let moderator_name = &moderator.name;

    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            warn!("Couldn't get guild ID");
            return Ok(());
        }
    };

    let warn_type = InfractionType::Warn.as_str();

    let warnings = match infractions::infractions(user_id, guild_id, warn_type, pool).await {
        Ok(warnings) => warnings,
        Err(why) => {
            error!("Couldn't get warnings: {why:?}");
            return Ok(());
        }
    };

    let number_of_warnings = warnings.len();
    if number_of_warnings < 1 {
        let reply = messages::warn_reply(format!("<@{user_id}> hasn't been warned before."));
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    } else {
        for warning in warnings {
            let case_id = warning.0;
            if case_id != id {
                continue;
            }

            let mut user_infractions = match users::infractions(user_id, guild_id, pool).await {
                Some(infractions) => infractions,
                None => {
                    warn!("Couldn't get infractions for @{user_name}");
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
                pool,
            )
            .await;

            infractions::delete_infraction(id, warn_type, pool).await;

            if let Some(reason) = reason.clone() {
                let number_of_reason = reason.chars().count();
                if number_of_reason < 6 || number_of_reason > 80 {
                    let reply = messages::warn_reply("Reason must be between 8 and 80 characters.");
                    if let Err(why) = ctx.send(reply).await {
                        error!("Couldn't send reply: {why:?}");
                    }

                    return Ok(());
                }

                info!("@{user_name} unwarned by @{moderator_name}: {reason}");
            } else {
                info!("@{user_name} unwarned by @{moderator_name}");
            }

            let reply = messages::ok_reply(format!("Removed warning from <@{user_id}>."));
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
            }

            break;
        }
    }

    Ok(())
}
