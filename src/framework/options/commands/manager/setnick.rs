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
    utility::{self, components::messages},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_NICKNAMES",
    guild_only,
    ephemeral
)]
/// Set a nickname for a user.
pub(crate) async fn setnick(
    ctx: Context<'_>,
    #[description = "The user to set the nickname for."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The nickname to set."]
    #[min_length = 1]
    #[max_length = 32]
    nickname: String,
) -> Result<(), Error> {
    let number_of_nickname = nickname.chars().count();
    if number_of_nickname < 1 || number_of_nickname > 32 {
        let reply = messages::warn_reply(
            format!("Nickname must be between 1 and 32 characters."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let guild_id = utility::guilds::guild_id(ctx).await;

    let user = utility::users::user(ctx, user_id).await;
    let user_name = &user.name;

    let moderator_name = &ctx.author().name;

    let mut member = utility::guilds::member(ctx, guild_id, user_id).await;
    let edit_member = EditMember::default().nickname(&nickname);

    if let Err(why) = member.edit(&ctx, edit_member).await {
        error!("Couldn't set @{user_name}'s nickname to {nickname:?}: {why:?}");

        let reply = messages::error_reply(format!("Couldn't set <@{user_id}>'s nickname."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Err(why.into());
    }

    info!("@{moderator_name} changed @{user_name}'s nickname to {nickname:?}");

    let reply = messages::ok_reply(
        format!("Changed <@{user_id}>'s nickname to {nickname}."),
        true,
    );
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
