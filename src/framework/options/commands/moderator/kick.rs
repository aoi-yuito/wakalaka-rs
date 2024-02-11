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
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "KICK_MEMBERS",
    required_bot_permissions = "KICK_MEMBERS | SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Kick a user outside.
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The user to kick."] user: User,
    #[description = "The reason for kicking."]
    #[min_length = 3]
    #[max_length = 80]
    reason: String,
) -> Result<(), Error> {
    if user.system {
        let reply = messages::error_reply(None, "Cannot kick system users!", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let user_id = user.id;

    let moderator = ctx.author();
    let moderator_id = moderator.id;
    if moderator_id == user_id {
        let reply = messages::error_reply(None, "Cannot kick yourself!", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let result = {
        let (user_name, user_mention) = (&user.name, user.mention());

        let (moderator_name, moderator_mention) = (&moderator.name, moderator.mention());

        let guild_id = models::guilds::guild_id(ctx)?;
        let guild_name = models::guilds::guild_name(ctx, guild_id);

        let member = guild_id.member(&ctx, user_id).await?;

        let message = messages::info_message(None, format!(
            "You've been kicked from {guild_name} by {moderator_mention} for {reason}.",
        ));
        user.direct_message(ctx, message).await?;

        match member.kick_with_reason(ctx, &reason).await {
            Ok(_) => {
                info!("@{moderator_name} kicked @{user_name} from {guild_name}: {reason}");
                Ok(format!("{user_mention} has been kicked."))
            }
            Err(why) => {
                error!("Failed to kick @{user_name}: {why:?}");
                Err(format!("An error occurred whilst kicking {user_mention}."))
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
