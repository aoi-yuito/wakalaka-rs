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
/// Unlock the door for a user.
pub async fn unban(
    ctx: Context<'_>,
    #[description = "The user to unban."] user: User,
    #[description = "The reason for unbanning, if any."]
    #[min_length = 3]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    if user.system {
        let reply = messages::error_reply(None, "Cannot unban system users!", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let (user_id, user_name, user_mention) = (user.id, &user.name, user.mention());

    let moderator = ctx.author();
    let (moderator_id, moderator_name) = (moderator.id, &moderator.name);

    if user_id == moderator_id {
        let reply = messages::error_reply(None, "Cannot unban yourself!", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let guild_id = models::guilds::guild_id(ctx)?;
    let guild_name = models::guilds::guild_name(ctx, guild_id);

    let mut user_infractions = users::select_infractions_from_users(&user_id, pool).await?;
    if user_infractions < 1 {
        let reply = messages::warn_reply(None, 
            format!("{user_mention} doesn't have any infractions!"),
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let bans = infractions::select_from_infractions(InfractionType::Ban, &user_id, &guild_id, pool)
        .await?;
    for ban in bans {
        let uuid = ban.0;

        if let Err(why) = guild_id.unban(ctx, user_id).await {
            error!("Failed to unban @{user_name}: {why:?}");

            let reply = messages::error_reply(None, 
                format!("An error occurred whilst unbanning {user_mention}."),
                true,
            );
            ctx.send(reply).await?;

            return Err(why.into());
        }

        guild_members::update_guilds_members_set_ban(&user_id, false, pool).await?;

        if let Some(ref reason) = reason {
            info!("@{moderator_name} unbanned @{user_name} from {guild_name}: {reason}");
        } else {
            info!("@{moderator_name} unbanned @{user_name} from {guild_name}")
        }

        infractions::delete_from_infractions(&uuid, &guild_id, pool).await?;

        user_infractions -= 1;
        if user_infractions < 0 {
            user_infractions = 0;
        }

        users::update_users_set_infractions(&user_id, user_infractions, pool).await?;

        let reply = messages::ok_reply(None, format!("{user_mention} has been unbanned."), true);
        ctx.send(reply).await?;
    }

    Ok(())
}
