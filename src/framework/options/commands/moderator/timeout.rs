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

use chrono::{Duration, Utc};
use serenity::{
    all::{Mentionable, User},
    model::Timestamp,
};
use tracing::{error, info};

use crate::{
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
    required_bot_permissions = "SEND_MESSAGES | MODERATE_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Give user a time-out.
pub async fn timeout(
    ctx: Context<'_>,
    #[description = "The user to time out."] user: User,
    #[description = "The reason for timing out."]
    #[min_length = 3]
    #[max_length = 80]
    reason: String,
    #[description = "The amount of days to time out for."]
    #[min = 1]
    #[max = 28]
    duration: Option<i64>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    if user.bot || user.system {
        let reply = messages::error_reply(None, "Cannot time out bots and system users!", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let user_id = user.id;

    let moderator = ctx.author();
    let moderator_id = moderator.id;
    if moderator_id == user_id {
        let reply = messages::error_reply(None, "Cannot time yourself out!", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let duration = duration.unwrap_or(1);

    let result = {
        let (user_name, user_mention) = (&user.name, user.mention());

        let (moderator_name, moderator_mention) = (&moderator.name, moderator.mention());

        let guild_id = models::guilds::guild_id(ctx)?;
        let guild_name = models::guilds::guild_name(ctx, guild_id);

        let created_at = Utc::now().naive_utc();

        let mut user_infractions = users::select_infractions_from_users(&user_id, pool).await?;

        let mut member = guild_id.member(&ctx, user_id).await?;

        let message = messages::info_message(None, format!(
            "You've been timed out in {guild_name} by {moderator_mention} for {reason}.",
        ));
        user.direct_message(ctx, message).await?;

        let time = Timestamp::from(Utc::now() + Duration::days(duration));
        let disabled_until = time
            .to_rfc3339()
            .expect("Failed to convert time to RFC3339");

        match member.disable_communication_until_datetime(ctx, time).await {
            Ok(_) => {
                guild_members::update_guilds_members_set_timeout(
                    &user_id,
                    true,
                    Some(disabled_until),
                    pool,
                )
                .await?;

                info!("@{moderator_name} timed out @{user_name} in {guild_name}: {reason}");

                infractions::insert_into_infractions(
                    InfractionType::Timeout,
                    &user_id,
                    &moderator_id,
                    &reason,
                    created_at,
                    &guild_id,
                    pool,
                )
                .await?;

                user_infractions += 1;

                users::update_users_set_infractions(&user_id, user_infractions, pool).await?;

                Ok(format!("{user_mention} has been timed out."))
            }
            Err(why) => {
                error!("Failed to time out @{user_name}: {why:?}");
                Err(format!(
                    "An error occurred whilst timing out {user_mention}."
                ))
            }
        }
    };

    let reply = match result {
        Ok(message) => messages::ok_reply(None, message, true),
        Err(message) => messages::error_reply(None, message, true),
    };
    ctx.send(reply).await?;

    Ok(())
}
