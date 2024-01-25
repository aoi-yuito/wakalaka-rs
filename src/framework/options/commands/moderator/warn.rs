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
use tracing::{error, info};

use crate::{
    check_restricted_guild_channel,
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
    user_cooldown = 5,
    ephemeral
)]
/// Warn a user for their misbehavior.
pub async fn warn(
    ctx: Context<'_>,
    #[description = "The user to warn."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for warning."]
    #[min_length = 6]
    #[max_length = 80]
    reason: String,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let pool = &ctx.data().pool;

    let user = models::users::user(ctx, user_id).await;
    if user.bot || user.system {
        let reply =
            messages::error_reply("Sorry, but bots and system users cannot be warned.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let reason_chars_count = reason.chars().count();
    if reason_chars_count < 6 || reason_chars_count > 80 {
        let reply = messages::info_reply(
            "Reason must be between `6` and `80` characters long.",
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
    let moderator_id = moderator.id;
    let moderator_name = &moderator.name;

    let (guild_id, guild_name) = (
        models::guilds::guild_id(ctx).await,
        models::guilds::guild_name(ctx).await,
    );

    let created_at = Utc::now().naive_utc();

    let mut user_infractions = users::select_infractions_from_users(&user_id, pool).await?;

    let warnings =
        infractions::select_from_infractions(InfractionType::Warn, &user_id, &guild_id, pool)
            .await?;

    let warning_count = warnings.len();
    if warning_count >= 3 {
        let reply = messages::warn_reply(
            format!(
            "<@{user_id}> has reached a maximum number of warnings. Take further action manually.",
        ),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let message = messages::info_message(format!(
        "You've been warned by <@{moderator_id}> in {guild_name} for {reason}.",
    ));
    if let Err(why) = user.direct_message(&ctx, message).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    info!("@{user_name} warned by @{moderator_name}: {reason}");

    infractions::insert_into_infractions(
        InfractionType::Warn,
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

    let reply = messages::ok_reply(format!("<@{user_id}> has been warned."), true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
