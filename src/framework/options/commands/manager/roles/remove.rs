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

use serenity::all::{Role, UserId};
use tracing::{error, info};

use crate::{
    check_restricted_guild_channel,
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Take role(s) from a user.
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The role(s) to take."] roles: Vec<Role>,
    #[description = "The user to take the role(s) from."]
    #[rename = "user"]
    user_id: UserId,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let result = {
        let role_ids = models::roles::role_ids(roles).await;

        let user_name = models::users::user_name(ctx, user_id).await;

        let guild = models::guilds::guild(ctx).await;
        let (guild_id, guild_name) = (guild.id, &guild.name);

        let member = models::members::member(ctx, guild_id, user_id).await;

        match member.remove_roles(&ctx, &role_ids).await {
            Ok(_) => {
                info!("Removed role(s) from @{user_name} in {guild_name}");
                Ok(format!("I've removed role(s) from <@{user_id}>."))
            }
            Err(why) => {
                error!("Couldn't remove role(s) from @{user_name} in {guild_name}: {why:?}");
                Err(format!(
                    "Sorry, but I couldn't remove role(s) from <@{user_id}>."
                ))
            }
        }
    };

    let reply = match result {
        Ok(message) => messages::ok_reply(message, true),
        Err(message) => messages::error_reply(message, true),
    };
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
