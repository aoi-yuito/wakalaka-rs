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
    utility::{self, components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    guild_only,
    ephemeral
)]
/// Customise an existing role.
pub(crate) async fn edit(
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
    if name.is_some() {
        let number_of_name = name.as_ref().unwrap().chars().count();
        if number_of_name < 1 || number_of_name > 100 {
            let reply = messages::warn_reply(
                format!("I'm afraid the name has to be between `1` and `100` characters."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        }
    }

    let role_name = role.name.clone();

    let guild = models::guilds::guild(ctx).await;
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

    if let Err(why) = role.edit(ctx, role_builder).await {
        error!("Couldn't alter @{role_name} role in {guild_name}: {why:?}");

        let reply = messages::error_reply(
            format!("Sorry, but I couldn't alter a role called `{role_name}`."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Err(why.into());
    }

    info!("Altered @{role_name} role in {guild_name}");

    let reply = messages::ok_reply(format!("I've altered a role called `{role_name}`."), true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
