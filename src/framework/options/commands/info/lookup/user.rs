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

use crate::{
    utility::components::{embeds, replies},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Get information about a user.
pub async fn user(
    ctx: Context<'_>,
    #[description = "The user to get information of."] user: User,
) -> Result<(), Error> {
    let (user_id, user_name, user_avatar_url, user_accent_colour) = (
        &user.id,
        &user.name,
        &user.avatar_url().unwrap_or(user.default_avatar_url()),
        &user.accent_colour,
    );

    let embed =
        embeds::lookup_user_command_embed(user_id, user_name, user_avatar_url, user_accent_colour);

    let reply = replies::reply_embed(embed, true);
    ctx.send(reply).await?;

    Ok(())
}
