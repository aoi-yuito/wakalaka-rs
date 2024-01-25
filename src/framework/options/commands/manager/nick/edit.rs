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
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let nickname_char_count = nickname.chars().count();
    if nickname_char_count < 1 || nickname_char_count > 32 {
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
    let user_id = user.id;
    let user_name = &user.name;

    let moderator_name = &ctx.author().name;

    let mut member = models::members::member(ctx, guild_id, user_id).await;
    let member_builder = EditMember::default().nickname(nickname.clone());

    if let Err(why) = member.edit(ctx, member_builder).await {
        error!("Couldn't edit @{user_name}'s nickname to {nickname:?}: {why:?}");

        let reply = messages::error_reply(
            format!("Sorry, but I couldn't change <@{user_id}>'s nickname to `{nickname}`."),
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
        format!("I've changed <@{user_id}>'s nickname to `{nickname}`."),
        true,
    );
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
