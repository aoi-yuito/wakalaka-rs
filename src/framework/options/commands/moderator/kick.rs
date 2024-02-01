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

use serenity::all::User;
use tracing::{error, info};

use crate::{
    check_restricted_guild_channel,
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "KICK_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Kick a user outside.
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The user to kick."] user: User,
    #[description = "The reason for kicking."] reason: String,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let user_id = user.id;

    let moderator = models::users::author(ctx)?;
    let moderator_id = moderator.id;

    if user.system {
        let reply = messages::error_reply("Sorry, but system users cannot be kicked.", true);
        ctx.send(reply).await?;

        return Ok(());
    }
    if user_id == moderator_id {
        let reply = messages::error_reply("Sorry, but you cannot ban yourself.", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let reason_char_count = reason.chars().count();
    if reason_char_count < 6 || reason_char_count > 80 {
        let reply =
            messages::info_reply("Reason must be between `6` and `80` characters long.", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let result = {
        let (user_name, user_mention) =
            (&user.name, models::users::user_mention(ctx, user_id).await?);

        let (moderator_name, moderator_mention) =
            (&moderator.name, models::users::author_mention(ctx)?);

        let (guild_id, guild_name) = (
            models::guilds::guild_id(ctx)?,
            models::guilds::guild_name(ctx)?,
        );

        let member = models::members::member(ctx, guild_id, user_id).await?;

        let message = messages::info_message(format!(
            "You've been kicked from {guild_name} by {moderator_mention} for {reason}.",
        ));
        if let Err(why) = user.direct_message(ctx, message).await {
            return Err(format!("Couldn't send direct message: {why:?}").into());
        }

        match member.kick_with_reason(ctx, &reason).await {
            Ok(_) => {
                info!("@{moderator_name} kicked @{user_name} from {guild_name}: {reason}");
                Ok(format!("{user_mention} has been kicked."))
            }
            Err(why) => {
                error!("Couldn't kick @{user_name}: {why:?}");
                Err(format!("Sorry, but I couldn't kick {user_mention}."))
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
