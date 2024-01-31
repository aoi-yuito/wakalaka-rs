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

use serenity::{all::Role, builder::EditRole};
use tracing::{error, info};

use crate::{
    check_restricted_guild_channel,
    utility::{self, components::messages, models},
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
/// Customise an existing role.
pub async fn edit(
    ctx: Context<'_>,
    #[description = "The role to customise."] mut role: Role,
    #[description = "The name for the role, if any."]
    #[min_length = 1]
    #[max_length = 100]
    name: Option<String>,
    #[description = "The colour of the role in hexadecimal, if any."]
    #[min = 3]
    #[max = 11]
    colour: Option<String>,
    #[description = "Whether the role should be pinned above lesser roles."] hoist: Option<bool>,
    #[description = "Whether the role should be mentionable."] mentionable: Option<bool>,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    if name.is_some() {
        let name_char_count = name.as_ref().unwrap().chars().count();
        if name_char_count < 1 || name_char_count > 100 {
            let reply = messages::info_reply(
                format!("Name of the role must be between `1` and `100` characters long."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        }
    }

    let result = {
        let role_name = models::roles::role_name(&role).clone();

        let guild = models::guilds::guild(ctx)?;
        let guild_name = &guild.name;

        let role_builder = if let Some(colour) = colour {
            let colour = utility::hex_to_u32(&colour);

            EditRole::new()
                .name(&name.unwrap_or(role_name.clone()))
                .colour(colour)
                .hoist(hoist.is_some())
                .mentionable(mentionable.is_some())
        } else {
            EditRole::new()
                .name(&name.unwrap_or(role_name.clone()))
                .hoist(hoist.is_some())
                .mentionable(mentionable.is_some())
        };

        match role.edit(ctx, role_builder).await {
            Ok(_) => {
                info!("Altered @{role_name} role in {guild_name}");
                Ok(format!("I've altered a role called `{role_name}`."))
            }
            Err(why) => {
                error!("Couldn't alter @{role_name} role in {guild_name}: {why:?}");
                Err(format!(
                    "Sorry, but I couldn't alter a role called `{role_name}`."
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
