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
    utility::{self, messages},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "KICK_MEMBERS",
    guild_only,
    ephemeral
)]
/// Kick a user outside.
pub(crate) async fn kick(
    ctx: Context<'_>,
    #[description = "The user to kick."] user: User,
    #[description = "The reason for kicking. (6-80)"] reason: String,
) -> Result<(), Error> {
    if user.system {
        let reply = messages::error_reply("Cannot kick system users.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(Error::from(why));
        }

        return Ok(());
    }

    let number_of_reason = reason.chars().count();
    if number_of_reason < 6 || number_of_reason > 80 {
        let reply = messages::warn_reply("Reason must be between 8 and 80 characters.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(Error::from(why));
        }

        return Ok(());
    }

    let (user_id, user_name) = (user.id, &user.name);

    let moderator = ctx.author();
    let (moderator_id, moderator_name) = (moderator.id, &moderator.name);

    let (guild_id, guild_name) = (
        utility::guilds::guild_id(ctx).await,
        utility::guilds::guild_name(ctx).await,
    );

    let member = utility::guilds::member(ctx, guild_id, user_id).await;

    let message = messages::message(format!(
        "You've been kicked from {guild_name} by <@{moderator_id}> for {reason}.",
    ));
    if let Err(why) = user.direct_message(&ctx, message).await {
        error!("Couldn't send reply: {why:?}");
    }

    if let Err(why) = member.kick_with_reason(&ctx, &reason).await {
        error!("Couldn't kick @{user_name}: {why:?}");

        let reply = messages::error_reply("Couldn't kick member.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(Error::from(why));
        }

        return Err(Error::from(why));
    } else {
        info!("@{moderator_name} kicked @{user_name} from {guild_name}: {reason}");

        let reply = messages::ok_reply(format!("<@{user_id}> has been kicked."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(Error::from(why));
        }
    }

    Ok(())
}
