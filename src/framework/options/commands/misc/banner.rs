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
use tracing::{error, warn};

use crate::{
    check_restricted_guild_channel,
    utility::components::{embeds, messages, replies},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    context_menu_command = "Get Banner",
    category = "Misc",
    user_cooldown = 5,
    guild_only
)]
/// Get a user's banner.
pub async fn banner(
    ctx: Context<'_>,
    #[description = "The user to get the banner from."] user: User,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let http = ctx.http();

    let user_id = user.id;

    let user_raw = match http.get_user(user_id).await {
        Ok(user) => user,
        Err(why) => {
            error!("Couldn't get user: {why:?}");
            return Err(why.into());
        }
    };

    let user_name = &user.name;
    let (user_avatar_url, user_banner_url) = (
        user.avatar_url().unwrap_or(user_raw.default_avatar_url()),
        match user_raw.banner_url() {
            Some(url) => url,
            None => {
                warn!("Couldn't find @{user_name}'s banner");

                let reply = messages::error_reply(
                    format!("Sorry, but I couldn't find <@{user_id}>'s banner."),
                    true,
                );
                if let Err(why) = ctx.send(reply).await {
                    error!("Couldn't send reply: {why:?}");
                    return Err(why.into());
                }

                return Ok(());
            }
        },
    );

    let embed = embeds::banned_command_embed(user_name, user_avatar_url, user_banner_url);

    let reply = replies::reply_embed(embed, false);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
