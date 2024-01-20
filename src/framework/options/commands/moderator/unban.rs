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
    utility::{self, messages},
    Context, Error,
};

/// Unlock the door for a user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "BAN_MEMBERS",
    guild_only,
    ephemeral
)]
pub(crate) async fn unban(
    ctx: Context<'_>,
    #[description = "The user to unban."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for unbanning, if any. (6-80)"] reason: Option<String>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user = utility::user(user_id, ctx).await;
    let user_name = &user.name;

    let moderator = ctx.author();
    let moderator_name = &moderator.name;

    let (guild_id, guild_name) = (utility::guild_id(ctx), utility::guild_name(ctx));

    let ban_type = InfractionType::Ban.as_str();

    let infractions = infractions::infractions(user_id, guild_id, ban_type, pool).await?;

    let number_of_infractions = infractions.len();
    if number_of_infractions < 1 {
        let reply = messages::warn_reply(format!("<@{user_id}> hasn't been punished before."));
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
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

        if let Err(why) = guild_id.unban(&ctx, user_id).await {
            error!("Couldn't unban member: {why:?}");
            return Ok(());
        }

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

        infractions::delete_infraction(case_id, ban_type, pool).await;

        if let Some(reason) = reason.clone() {
            let number_of_reason = reason.chars().count();
            if number_of_reason < 6 || number_of_reason > 80 {
                let reply = messages::warn_reply("Reason must be between 8 and 80 characters.");
                if let Err(why) = ctx.send(reply).await {
                    error!("Couldn't send reply: {why:?}");
                }

                return Ok(());
            }

            info!("@{user_name} unbanned from {guild_name} by @{moderator_name}: {reason}");
        } else {
            info!("@{user_name} unbanned from {guild_name} by @{moderator_name}")
        }

        let reply = messages::ok_reply(format!("<@{user_id}> has been unbanned."));
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }
    }

    Ok(())
}
