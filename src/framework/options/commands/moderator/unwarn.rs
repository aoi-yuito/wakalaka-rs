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
use tracing::error;

use crate::{
    database::infractions::{self, InfractionType},
    utility::messages,
    Context, Error,
};

/// Remove a warning from a specific user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    guild_only,
    ephemeral
)]
pub(crate) async fn unwarn(
    ctx: Context<'_>,
    #[description = "The user to unwarn."] user: User,
    #[description = "ID of the warning to delete."] id: i32,
) -> Result<(), Error> {
    if user.bot || user.system {
        let reply = messages::error_reply("Can't remove warnings from bots or system users");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let pool = &ctx.data().pool;

    let user_id = user.id;

    if id < 1 {
        let reply = messages::warn_reply("Case ID must be greater than 0");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let infraction_type = InfractionType::Warn.as_str();

    infractions::delete_infraction(id, infraction_type, pool).await;

    let reply = messages::success_reply(format!("Removed warning {id} from <@{user_id}>"));
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
    }

    Ok(())
}
