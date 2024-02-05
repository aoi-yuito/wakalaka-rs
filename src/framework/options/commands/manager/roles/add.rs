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

use serenity::builder::EditRole;
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
    required_bot_permissions = "SEND_MESSAGES | MANAGE_ROLES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Create a new role.
pub async fn add(
    ctx: Context<'_>,
    #[description = "The name of the role."]
    #[min_length = 1]
    #[max_length = 100]
    name: String,
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

    let name_char_count = name.chars().count();
    if name_char_count < 1 || name_char_count > 100 {
        let reply = messages::info_reply(
            format!("Name of the role must be between `1` and `100` characters long."),
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let result = {
        let guild = models::guilds::guild(ctx)?;
        let guild_name = &guild.name;

        let role_builder = if let Some(colour) = colour {
            let colour = utility::hex_to_u32(&colour);

            EditRole::new()
                .name(&name)
                .colour(colour)
                .hoist(hoist.is_some())
                .mentionable(mentionable.is_some())
        } else {
            EditRole::new()
                .name(&name)
                .hoist(hoist.is_some())
                .mentionable(mentionable.is_some())
        };

        match guild.create_role(ctx, role_builder).await {
            Ok(_) => {
                info!("Created role called @{name} in {guild_name}");
                Ok(format!("I've created a role called `{name}`."))
            }
            Err(why) => {
                error!("Couldn't create role called @{name} in {guild_name}: {why:?}");
                Err(format!(
                    "Sorry, but I couldn't create a role called `{name}`."
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
