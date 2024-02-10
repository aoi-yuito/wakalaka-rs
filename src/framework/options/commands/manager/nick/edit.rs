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

use serenity::{all::UserId, builder::EditMember};
use tracing::{error, info};

use crate::{
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_NICKNAMES",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_NICKNAMES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Change a user's nickname.
pub async fn edit(
    ctx: Context<'_>,
    #[description = "The user to change the nickname of."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The nickname to set."]
    #[min_length = 1]
    #[max_length = 32]
    nickname: String,
) -> Result<(), Error> {
    let guild_id = models::guilds::guild_id(ctx)?;

    let user = models::users::user(ctx, user_id).await?;
    let (user_id, user_name, user_mention) = (
        user.id,
        user.name,
        models::users::user_mention(ctx, user_id).await?,
    );

    let mut member = models::members::member(ctx, guild_id, user_id).await?;
    let member_builder = EditMember::default().nickname(&nickname);

    let result = match member.edit(ctx, member_builder).await {
        Ok(_) => {
            let moderator_name = models::users::author_name(ctx)?;

            info!("@{moderator_name} changed @{user_name}'s nickname to {nickname:?}");
            Ok(format!(
                "Changed {user_mention}'s nickname to `{nickname}`."
            ))
        }
        Err(why) => {
            error!("Couldn't edit @{user_name}'s nickname to {nickname:?}: {why:?}");
            Err(format!(
                "Sorry, but I couldn't change {user_mention}'s nickname to `{nickname}`."
            ))
        }
    };

    let reply = match result {
        Ok(message) => messages::ok_reply(message, true),
        Err(message) => messages::error_reply(message, true),
    };
    ctx.send(reply).await?;

    Ok(())
}
