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

use serenity::{all::UserId, builder::EditMember};
use tracing::{error, info, warn};

use crate::{
    database::{
        infractions::{self, InfractionType},
        users,
    },
    utility::{self, messages},
    Context, Error,
};

/// Allow a user to speak in voice channels.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MUTE_MEMBERS",
    guild_only,
    ephemeral
)]
pub(crate) async fn unmute(
    ctx: Context<'_>,
    #[description = "The user to unmute."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for unmute, if any. (6-80)"] reason: Option<String>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user = utility::user(user_id, ctx).await;
    if user.bot || user.system {
        let reply = messages::error_reply("Cannot unmute bots or system users.");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let user_name = &user.name;

    let moderator = ctx.author();
    let moderator_name = &moderator.name;

    let guild_id = utility::guild_id(ctx);

    let mute_type = InfractionType::Mute.as_str();

    let infractions = match infractions::infractions(user_id, guild_id, mute_type, pool).await {
        Ok(infractions) => infractions,
        Err(why) => {
            error!("Couldn't get infractions from database: {why:?}");
            return Ok(());
        }
    };

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
                warn!("Couldn't get infractions for @{user_name}");
                return Ok(());
            }
        };

        let mut member = match guild_id.member(&ctx, user_id).await {
            Ok(member) => member,
            Err(why) => {
                error!("Couldn't get member: {why:?}");
                return Ok(());
            }
        };
        let edit_member = EditMember::default().mute(false);

        if let Err(why) = member.edit(&ctx, edit_member).await {
            error!("Couldn't unmute member: {why:?}");

            let reply = messages::error_reply("Couldn't unmute member.");
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
            }

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

        infractions::delete_infraction(case_id, mute_type, pool).await;

        if let Some(reason) = reason.clone() {
            let number_of_reason = reason.chars().count();
            if number_of_reason < 6 || number_of_reason > 80 {
                let reply = messages::warn_reply("Reason must be between 8 and 80 characters.");
                if let Err(why) = ctx.send(reply).await {
                    error!("Couldn't send reply: {why:?}");
                }

                return Ok(());
            }

            info!("@{user_name} unmuted by @{moderator_name}: {reason}");
        } else {
            info!("@{user_name} unmuted by @{moderator_name}");
        }

        let reply = messages::ok_reply(format!("<@{user_id}> has been unmuted."));
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }
    }

    Ok(())
}
