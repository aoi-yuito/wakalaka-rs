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

use chrono::Utc;
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

/// Disallow a user from interaction in voice channels.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "DEAFEN_MEMBERS",
    guild_only,
    ephemeral
)]
pub(crate) async fn deafen(
    ctx: Context<'_>,
    #[description = "The user to deafen."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for deafening. (6-80)"] reason: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    
    let user = utility::user(user_id, ctx).await;
    if user.bot || user.system {
        let reply = messages::error_reply("Cannot deafen bots or system users.");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let number_of_reason = reason.chars().count();
    if number_of_reason < 6 || number_of_reason > 80 {
        let reply = messages::warn_reply("Reason must be between 8 and 80 characters.");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let user_name = &user.name;

    let moderator = ctx.author();
    let (moderator_id, moderator_name) = (moderator.id, &moderator.name);

    let (guild_id, guild_name) = (utility::guild_id(ctx), utility::guild_name(ctx));

    let created_at = Utc::now().naive_utc();

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
            error!("Couldn't get member: {why:?}");
            return Ok(());
        }
    };
    let edit_member = EditMember::default().deafen(true);

    let message = messages::message(format!(
        "You've been deafened by <@{moderator_id}> in {guild_name} for {reason}.",
    ));
    if let Err(why) = user.direct_message(&ctx, message).await {
        error!("Couldn't send reply: {why:?}");
    }

    if let Err(why) = member.edit(&ctx, edit_member).await {
        error!("Couldn't deafen member: {why:?}");

        let reply = messages::error_reply("Couldn't deafen member.");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    user_infractions += 1;

    users::update_user(
        user_id,
        guild_id,
        user_infractions,
        true,
        false,
        false,
        false,
        pool,
    )
    .await;

    infractions::insert_infraction(
        user_id,
        deaf_type,
        moderator_id,
        guild_id,
        &reason,
        Some(created_at),
        pool,
    )
    .await;

    info!("@{moderator_name} deafened @{user_name} in {guild_name}: {reason}");

    let reply = messages::ok_reply(format!("<@{user_id}> has been deafened.",));
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
    }

    Ok(())
}
