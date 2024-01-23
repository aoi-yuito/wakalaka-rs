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
    check_restricted_guild_channel, utility::{self, components::messages, models}, Context, Error
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    guild_only,
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
    let restricted = check_restricted_guild_channel!(ctx);
    if restricted {
        return Ok(());
    }
    
    let number_of_name = name.chars().count();
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

    let guild = models::guilds::guild(ctx).await;
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

    if let Err(why) = guild.create_role(ctx, role_builder).await {
        error!("Couldn't create @{name} role in {guild_name}: {why:?}");

        let reply = messages::error_reply(
            format!("Sorry, but I couldn't create a role called `{name}`."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Err(why.into());
    }

    info!("Created @{name} role in {guild_name}");

    let reply = messages::ok_reply(format!("I've created a role called `{name}`."), true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
