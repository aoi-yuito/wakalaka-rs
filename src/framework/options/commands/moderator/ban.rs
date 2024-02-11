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
use serenity::all::{Mentionable, User};
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
    required_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS | SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Lock the door for a user.
pub async fn ban(
    ctx: Context<'_>,
    #[description = "The user to ban."] user: User,
    #[description = "The reason for banning."]
    #[min_length = 3]
    #[max_length = 80]
    reason: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    if user.system {
        let reply = messages::error_reply("Cannot ban system users!", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let user_id = user.id;

    let moderator = ctx.author();
    let moderator_id = moderator.id;
    if moderator_id == user_id {
        let reply = messages::error_reply("Cannot ban yourself!", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let result = {
        let (user_name, user_mention) = (&user_id.to_user(ctx).await?.name, user_id.mention());

        let (moderator_name, moderator_mention) = (&moderator.name, moderator.mention());

        let guild_id = models::guilds::guild_id(ctx)?;
        let guild_name = models::guilds::guild_name(ctx, guild_id);

        let created_at = Utc::now().naive_utc();

        let mut user_infractions = users::select_infractions_from_users(&user_id, pool).await?;

        let message = messages::info_message(format!(
            "You've been banned from {guild_name} by {moderator_mention} for {reason}.",
        ));
        user.direct_message(ctx, message).await?;

        match guild_id.ban_with_reason(ctx, user_id, 0, &reason).await {
            Ok(_) => {
                guild_members::update_guilds_members_set_ban(&user_id, true, pool).await?;

                infractions::insert_into_infractions(
                    InfractionType::Ban,
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

                info!("@{moderator_name} banned @{user_name} from {guild_name}: {reason}");
                Ok(format!("{user_mention} has been banned."))
            }
            Err(why) => {
                error!("Failed to ban @{user_name}: {why:?}");
                Err(format!("An error occurred whilst banning {user_mention}."))
            }
        }
    };

    let reply = match result {
        Ok(message) => messages::ok_reply(message, true),
        Err(message) => messages::error_reply(message, true),
    };
    ctx.send(reply).await?;

    Ok(())
}
