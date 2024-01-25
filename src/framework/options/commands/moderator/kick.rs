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

    if user.system {
        let reply = messages::error_reply("Sorry, but system users cannot be kicked.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let reason_chars_count = reason.chars().count();
    if reason_chars_count < 6 || reason_chars_count > 80 {
        let reply = messages::info_reply("Reason must be between `6` and `80` characters long.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let (user_id, user_name) = (user.id, &user.name);

    let moderator = ctx.author();
    let (moderator_id, moderator_name) = (moderator.id, &moderator.name);

    let (guild_id, guild_name) = (
        models::guilds::guild_id(ctx).await,
        models::guilds::guild_name(ctx).await,
    );

    let member = models::members::member(ctx, guild_id, user_id).await;

    let message = messages::info_message(format!(
        "You've been kicked from {guild_name} by <@{moderator_id}> for {reason}.",
    ));
    if let Err(why) = user.direct_message(&ctx, message).await {
        error!("Couldn't send reply: {why:?}");
    }

    if let Err(why) = member.kick_with_reason(&ctx, &reason).await {
        error!("Couldn't kick @{user_name}: {why:?}");

        let reply =
            messages::error_reply(format!("Sorry, but I couldn't kick <@{user_id}>."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Err(why.into());
    } else {
        info!("@{moderator_name} kicked @{user_name} from {guild_name}: {reason}");

        let reply = messages::ok_reply(format!("<@{user_id}> has been kicked."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }
    }

    Ok(())
}
