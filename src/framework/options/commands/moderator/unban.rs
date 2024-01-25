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
use tracing::{error, info};

use crate::{
    check_restricted_guild_channel,
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
    required_permissions = "BAN_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Unlock the door for a user.
pub async fn unban(
    ctx: Context<'_>,
    #[description = "The user to unban."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for unbanning, if any."]
    #[min_length = 6]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let pool = &ctx.data().pool;

    let user = models::users::user(ctx, user_id).await;
    let user_name = &user.name;

    let moderator = ctx.author();
    let moderator_name = &moderator.name;

    let (guild_id, guild_name) = (
        models::guilds::guild_id(ctx).await,
        models::guilds::guild_name(ctx).await,
    );

    let mut user_infractions = users::select_infractions_from_users(&user_id, pool).await?;
    if user_infractions < 1 {
        let reply =
            messages::info_reply(format!("<@{user_id}> hasn't been punished before."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let bans = infractions::select_from_infractions(InfractionType::Ban, &user_id, &guild_id, pool)
        .await?;
    for ban in bans {
        let uuid = ban.0;

        if let Err(why) = guild_id.unban(&ctx, user_id).await {
            error!("Couldn't unban @{user_name}: {why:?}");

            let reply =
                messages::error_reply(format!("Sorry, but I couldn't unban <@{user_id}>."), true);
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Err(why.into());
        }

        guild_members::update_guilds_members_set_ban(&user_id, false, pool).await?;

        if let Some(reason) = reason.clone() {
            let reason_chars_count = reason.chars().count();
            if reason_chars_count < 6 || reason_chars_count > 80 {
                let reply =
                    messages::info_reply("Reason must be between `6` and `80` characters long.", true);
                if let Err(why) = ctx.send(reply).await {
                    error!("Couldn't send reply: {why:?}");
                    return Err(why.into());
                }

                return Ok(());
            }

            info!("@{user_name} unbanned from {guild_name} by @{moderator_name}: {reason}");
        } else {
            info!("@{user_name} unbanned from {guild_name} by @{moderator_name}")
        }

        infractions::delete_from_infractions(&uuid, &guild_id, pool).await?;

        user_infractions -= 1;
        if user_infractions < 0 {
            user_infractions = 0;
        }

        users::update_users_set_infractions(&user_id, user_infractions, pool).await?;

        let reply = messages::ok_reply(format!("<@{user_id}> has been unbanned."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }
    }

    Ok(())
}
