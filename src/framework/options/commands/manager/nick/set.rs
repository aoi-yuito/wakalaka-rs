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
    check_restricted_guild_channel,
    utility::{components::messages, models},
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
pub async fn set(
    ctx: Context<'_>,
    #[description = "The user to set the nickname for."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The nickname to set."]
    #[min_length = 1]
    #[max_length = 32]
    nickname: String,
) -> Result<(), Error> {
    let restricted = check_restricted_guild_channel!(ctx);
    if restricted {
        return Ok(());
    }

    let nickname_chars_count = nickname.chars().count();
    if nickname_chars_count < 1 || nickname_chars_count > 32 {
        let reply = messages::info_reply(
            format!("Nickname must be between `1` and `32` characters long."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let guild_id = models::guilds::guild_id(ctx).await;

    let user = models::users::user(ctx, user_id).await;
    let user_name = &user.name;

    let moderator_name = &ctx.author().name;

    let mut member = models::members::member(ctx, guild_id, user_id).await;
    let edit_member = EditMember::default().nickname(&nickname);

    if let Err(why) = member.edit(&ctx, edit_member).await {
        error!("Couldn't change @{user_name}'s nickname to {nickname:?}: {why:?}");

        let reply = messages::error_reply(
            format!("Sorry, but I couldn't change <@{user_id}>'s nickname."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Err(why.into());
    }

    info!("@{moderator_name} changed @{user_name}'s nickname to {nickname:?}");

    let reply = messages::ok_reply(
        format!("I've changed <@{user_id}>'s nickname to {nickname}."),
        true,
    );
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
