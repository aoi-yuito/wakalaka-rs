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

/// Warn a user for their misbehavior.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    guild_only,
    ephemeral
)]
pub(crate) async fn warn(
    ctx: Context<'_>,
    #[description = "The user to warn."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for warning. (6-80)"] reason: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    
    let user = utility::user(user_id, ctx).await;
    if user.bot || user.system {
        let reply = messages::error_reply("Cannot warn bots or system users.");
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
    let moderator_id = moderator.id;
    let moderator_name = &moderator.name;

    let (guild_id, guild_name) = (utility::guild_id(ctx), utility::guild_name(ctx));

    let created_at = Utc::now().naive_utc();

    let warn_type = InfractionType::Warn.as_str();

    let mut user_infractions = match users::infractions(user_id, guild_id, pool).await {
        Some(infractions) => infractions,
        None => {
            warn!("Couldn't get infractions for @{user_name} in database");
            return Ok(());
        }
    };

    let infractions = infractions::infractions(user_id, guild_id, warn_type, pool).await?;

    let number_of_infractions = infractions
        .iter()
        .filter(|warning| warning.1 == warn_type)
        .count();
    if number_of_infractions >= 3 {
        let reply = messages::warn_reply(format!(
            "<@{user_id}> has reached a maximum number of warnings. Take further action manually.",
        ));
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    } else {
        let message = messages::message(format!(
            "You've been warned by <@{moderator_id}> in {guild_name} for {reason}.",
        ));
        if let Err(why) = user.direct_message(&ctx, message).await {
            error!("Couldn't send reply: {why:?}");
        }

        user_infractions += 1;

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

        infractions::insert_infraction(
            user_id,
            warn_type,
            moderator_id,
            guild_id,
            &reason,
            Some(created_at),
            pool,
        )
        .await;

        info!("@{user_name} warned by @{moderator_name}: {reason}");

        let reply = messages::ok_reply(format!("<@{user_id}> has been warned.",));
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }
    }

    Ok(())
}
