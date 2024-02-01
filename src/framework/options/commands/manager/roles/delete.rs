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

use serenity::all::Role;
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
/// Delete an existing role.
pub async fn delete(
    ctx: Context<'_>,
    #[description = "The role to delete."] mut role: Role,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let result = {
        let role_name = models::roles::role_name(&role).clone();

        let guild = models::guilds::guild(ctx)?;
        let guild_name = &guild.name;

        match role.delete(ctx).await {
            Ok(_) => {
                info!("Deleted role called @{role_name} from {guild_name}");
                Ok(format!("I've deleted a role called `{role_name}`."))
            }
            Err(why) => {
                error!("Couldn't delete role called @{role_name} from {guild_name}: {why:?}");
                Err(format!(
                    "Sorry, but I couldn't delete a role called `{role_name}`."
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
