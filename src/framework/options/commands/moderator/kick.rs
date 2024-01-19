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
use tracing::{error, info, warn};

use crate::{utility::messages, Context, Error};

/// Kick a user outside.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "KICK_MEMBERS",
    guild_only,
    ephemeral
)]
pub(crate) async fn kick(
    ctx: Context<'_>,
    #[description = "The user to kick."] user: User,
    #[description = "The reason for kicking. (6-80)"] reason: String,
) -> Result<(), Error> {
    if user.system {
        let reply = messages::error_reply("Cannot kick system users.");
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

    let member = match guild_id.member(&ctx, user_id).await {
        Ok(member) => member,
        Err(why) => {
            error!("Couldn't get member: {why:?}");
            return Ok(());
        }
    };

    match member.kick_with_reason(&ctx, &reason).await {
        Ok(_) => {
            if !user.bot {
                let message = messages::message(format!(
                    "You've been kicked from {guild_name} by <@{moderator_id}> for {reason}.",
                ));
                if let Err(why) = user.direct_message(&ctx, message).await {
                    error!("Couldn't send reply: {why:?}");
                }
            }
        }
        Err(why) => {
            error!("Couldn't kick member: {why:?}");

            let reply = messages::error_reply("Couldn't kick member.");
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
            }

            return Ok(());
        }
    }

    info!("@{moderator_name} kicked @{user_name} from {guild_name}: {reason}");

    let reply = messages::ok_reply(format!("<@{user_id}> has been kicked.",));
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
    }

    Ok(())
}
