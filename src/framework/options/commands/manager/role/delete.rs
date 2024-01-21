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
    utility::{self, components::messages},
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
/// Delete an existing role.
pub(crate) async fn delete(
    ctx: Context<'_>,
    #[description = "The role to delete."] mut role: Role,
) -> Result<(), Error> {
    let role_name = role.name.clone();

    let guild = utility::guilds::guild(ctx).await;
    let guild_name = &guild.name;

    if let Err(why) = role.delete(ctx).await {
        error!("Couldn't delete @{role_name} role from {guild_name}: {why:?}");

        let reply = messages::error_reply(
            format!("Couldn't delete a role called `{role_name}`."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    info!("Deleted @{role_name} role from {guild_name}");

    let reply = messages::ok_reply(format!("Deleted a role called `{role_name}`."), true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
