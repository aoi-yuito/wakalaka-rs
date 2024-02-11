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

use serenity::all::{Mentionable, User};
use tracing::info;

use crate::{
    database::{
        infractions::{self, InfractionType},
        users,
    },
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "SEND_MESSAGES | MODERATE_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Remove a specific warning from a user.
pub async fn unwarn(
    ctx: Context<'_>,
    #[description = "The user to unwarn."] user: User,
    #[description = "The warning to delete."]
    #[min_length = 36]
    #[max_length = 36]
    #[rename = "uuid"]
    other_uuid: String,
    #[description = "The reason for deleting warning, if any."]
    #[min_length = 3]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    if user.bot || user.system {
        let reply = messages::error_reply(None, 
            "Cannot remove warnings from bots and system users!",
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let (user_id, user_name, user_mention) = (user.id, &user.name, user.mention());

    let moderator = ctx.author();
    let (moderator_id, moderator_name) = (moderator.id, &moderator.name);
    if moderator_id == user_id {
        let reply = messages::error_reply(None, "Cannot unwarn yourself!", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let guild_id = models::guilds::guild_id(ctx)?;
    let guild_name = models::guilds::guild_name(ctx, guild_id);

    let mut user_infractions = users::select_infractions_from_users(&user_id, pool).await?;
    if user_infractions < 1 {
        let reply = messages::warn_reply(None, 
            format!("{user_mention} doesn't have any infractions!"),
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let warnings =
        infractions::select_from_infractions(InfractionType::Warn, &user_id, &guild_id, pool)
            .await?;
    for warning in warnings {
        let uuid = if warning.0 == other_uuid {
            warning.0
        } else {
            let reply = messages::warn_reply(None, 
                format!("{user_mention} doesn't have the following warning: `{other_uuid}`"),
                true,
            );
            ctx.send(reply).await?;

            return Ok(());
        };

        if let Some(ref reason) = reason {
            info!("@{user_name} unwarned by @{moderator_name} in {guild_name}: {reason}");
        } else {
            info!("@{user_name} unwarned by @{moderator_name} in {guild_name}");
        }

        infractions::delete_from_infractions(&uuid, &guild_id, pool).await?;

        user_infractions -= 1;
        if user_infractions < 0 {
            user_infractions = 0;
        }

        users::update_users_set_infractions(&user_id, user_infractions, pool).await?;

        let reply = messages::ok_reply(None, format!("Removed a warning from {user_mention}."), true);
        ctx.send(reply).await?;
    }

    Ok(())
}
