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
use tracing::warn;

use crate::{
    utility::{
        components::{embeds, messages, replies},
        models,
    },
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    context_menu_command = "Get Banner",
    category = "Misc",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5,
    guild_only
)]
/// Get a user's banner.
pub async fn banner(
    ctx: Context<'_>,
    #[description = "The user to get the banner from."] user: User,
) -> Result<(), Error> {
    let user_id = user.id;
    let (user_name, user_mention) = (&user.name, models::users::user_mention(ctx, user_id).await?);

    let (user_avatar_url, user_banner_url) = (
        user.avatar_url().unwrap_or(user.default_avatar_url()),
        match user.banner_url() {
            Some(banner_url) => banner_url,
            None => {
                warn!("Couldn't find @{user_name}'s banner");

                let reply = messages::error_reply(
                    format!("Sorry, but I couldn't find {user_mention}'s banner."),
                    true,
                );
                ctx.send(reply).await?;

                return Ok(());
            }
        },
    );

    let embed = embeds::banner_command_embed(user_name, user_avatar_url, user_banner_url);

    let reply = replies::reply_embed(embed, false);
    ctx.send(reply).await?;

    Ok(())
}
