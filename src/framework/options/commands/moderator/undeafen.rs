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

use serenity::{all::User, builder::EditMember};
use tracing::{error, info, warn};

use crate::{
    database::{
        infractions::{self, InfractionType},
        users,
    },
    utility::messages,
    Context, Error,
};

// Allow a user to speak/hear in voice channels.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "DEAFEN_MEMBERS",
    guild_only,
    ephemeral
)]
pub(crate) async fn undeafen(
    ctx: Context<'_>,
    #[description = "The user to undeafen."] user: User,
    #[description = "The reason for undeafening. (6-80)"] reason: Option<String>,
) -> Result<(), Error> {
    if user.bot || user.system {
        let reply = messages::error_reply("Can't undeafen bots or system users.");
        if let Err(why) = ctx.send(reply).await {
            warn!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let pool = &ctx.data().pool;

    let deaf_type = InfractionType::Deaf.as_str();

    let user_id = user.id;
    let user_name = &user.name;

    let moderator = ctx.author();
    let moderator_id = moderator.id;
    let moderator_name = &moderator.name;

    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            warn!("Couldn't get guild ID");
            return Ok(());
        }
    };
    let guild_name = match guild_id.name(&ctx.cache()) {
        Some(guild_name) => guild_name,
        None => {
            warn!("Couldn't get guild name");
            return Ok(());
        }
    };

    let deafens = match infractions::infractions(user_id, guild_id, deaf_type, pool).await {
        Ok(deafens) => deafens,
        Err(why) => {
            error!("Couldn't get deafens: {why:?}");
            return Ok(());
        }
    };

    let number_of_deafens = deafens.len();
    if number_of_deafens < 1 {
        let reply = messages::warn_reply(format!(
            "<@{user_id}> is not deafened in {guild_name}.",
            user_id = user_id,
            guild_name = guild_name
        ));
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    for deafen in deafens {
        let case_id = deafen.0;

        let deaf_type = InfractionType::Deaf.as_str();

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
                warn!("Couldn't get member: {why:?}");
                return Ok(());
            }
        };
        let edit_member = EditMember::default().deafen(false);

        if let Err(why) = member.edit(&ctx, edit_member).await {
            error!("Couldn't undeafen member: {why:?}");

            let reply = messages::error_reply("Couldn't undeafen member.");
            if let Err(why) = ctx.send(reply).await {
                warn!("Couldn't send reply: {why:?}");
            }

            return Ok(());
        }

        if let Some(reason) = reason.clone() {
            let number_of_reason = reason.chars().count();
            if number_of_reason < 6 || number_of_reason > 80 {
                let reply = messages::warn_reply("Reason must be between 8 and 80 characters.");
                if let Err(why) = ctx.send(reply).await {
                    error!("Couldn't send reply: {why:?}");
                }

                return Ok(());
            }

            info!("@{user_name} undeafened by @{moderator_name}: {reason}");

            let message = messages::message(format!(
                "You've been undeafened by <@{moderator_id}> in {guild_name} for {reason}.",
            ));
            if let Err(why) = user.direct_message(&ctx, message).await {
                warn!("Couldn't send reply: {why:?}");
            }
        } else {
            info!("@{user_name} undeafened by @{moderator_name}");

            let message = messages::message(format!(
                "You've been undeafened by <@{moderator_id}> in {guild_name}.",
            ));
            if let Err(why) = user.direct_message(&ctx, message).await {
                warn!("Couldn't send reply: {why:?}");
            }
        }

        user_infractions -= 1;

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

        infractions::delete_infraction(case_id, deaf_type, pool).await;

        let reply = messages::ok_reply(format!("<@{user_id}> has been undeafened."));
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }
    }

    Ok(())
}
