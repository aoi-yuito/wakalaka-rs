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
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "DEAFEN_MEMBERS",
    guild_only,
    ephemeral
)]
/// Allow a user to interact in voice channels.
pub(crate) async fn undeafen(
    ctx: Context<'_>,
    #[description = "The user to undeafen."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for undeafening, if any."]
    #[min_length = 6]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user = models::users::user(ctx, user_id).await;
    if user.bot || user.system {
        let reply = messages::error_reply(
            "Sorry, but bots and system users cannot be undeafened.",
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

    let deaf_type = InfractionType::Deaf.as_str();

    let infractions = infractions::infractions(user_id, guild_id, deaf_type, pool).await?;

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

        let mut user_infractions = match users::infractions(user_id, guild_id, pool).await {
            Some(infractions) => infractions,
            None => {
                warn!("Couldn't get infractions for @{user_name} in database");
                return Ok(());
            }
        };

        let mut member = models::guilds::member(ctx, guild_id, user_id).await;
        let edit_member = EditMember::default().deafen(false);

        if let Err(why) = member.edit(&ctx, edit_member).await {
            error!("Couldn't undeafen @{user_name}: {why:?}");

            let reply = messages::error_reply(
                format!("Sorry, but I couldn't undeafen <@{user_id}>."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Err(why.into());
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

        infractions::delete_infraction(case_id, deaf_type, pool).await;

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

            info!("@{user_name} undeafened by @{moderator_name}: {reason}");
        } else {
            info!("@{user_name} undeafened by @{moderator_name}")
        }

        let reply = messages::ok_reply(format!("<@{user_id}> has been undeafened."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }
    }

    Ok(())
}
