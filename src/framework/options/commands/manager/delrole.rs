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
pub(crate) async fn delrole(
    ctx: Context<'_>,
    #[description = "The name of the role to delete."] name: String,
) -> Result<(), Error> {
    let guild = utility::guilds::guild(ctx).await;
    let guild_name = &guild.name;

    let mut role = utility::roles::role(ctx, &name).await;
    if let Err(why) = role.delete(ctx).await {
        error!("Couldn't delete @{name} role: {why:?}");

        let reply = messages::error_reply(
            format!("Couldn't delete a role called `{name}`."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(Error::from(why));
        }

        return Ok(());
    } else {
        info!("Deleted @{name} role from {guild_name}");

        let reply = messages::ok_reply(format!("Deleted a role called `{name}`."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(Error::from(why));
        }
    }

    Ok(())
}
