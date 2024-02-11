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

use serenity::all::{Mentionable, Role, User};
use tracing::{error, info};

use crate::{
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_ROLES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Take role(s) from a user.
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The role(s) to take."] roles: Vec<Role>,
    #[description = "The user to take the role(s) from."] user: User,
) -> Result<(), Error> {
    let result = {
        let role_ids = models::roles::role_ids(roles).await;

        let (user_id, user_name, user_mention) = (user.id, &user.name, user.mention());

        let guild = models::guilds::guild(ctx)?;
        let (guild_id, guild_name) = (guild.id, &guild.name);

        let member = guild_id.member(&ctx, user_id).await?;

        match member.remove_roles(ctx, &role_ids).await {
            Ok(_) => {
                info!("Removed role(s) from @{user_name} in {guild_name}");
                Ok(format!("Removed role(s) from {user_mention}."))
            }
            Err(why) => {
                error!("Failed to remove role(s) from @{user_name} in {guild_name}: {why:?}");
                Err(format!(
                    "An error occurred whilst removing role(s) from {user_mention}."
                ))
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
