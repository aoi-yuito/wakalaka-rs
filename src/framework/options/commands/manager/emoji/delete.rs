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

use tracing::{error, info, warn};

use crate::{
    check_restricted_guild_channel,
    utility::{
        components::{self, messages},
        models,
    },
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "CREATE_GUILD_EXPRESSIONS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Delete an existing emoji.
pub async fn delete(
    ctx: Context<'_>,
    #[description = "The name of the emoji."]
    #[min_length = 2]
    #[max_length = 32]
    name: String,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let name_char_count = name.chars().count();
    if name_char_count < 2 || name_char_count > 32 {
        let reply = messages::info_reply(
            format!("Name of the emoji must be between `2` and `32` characters long."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let guild = models::guilds::guild(ctx).await;
    let guild_name = &guild.name;

    let emoji_id = match components::emojis::emoji_id(ctx, &name).await {
        Some(emoji_id) => emoji_id,
        None => {
            warn!("Couldn't find {name:?} emoji in {guild_name}");

            let reply = messages::error_reply(
                format!("Sorry, but I couldn't find an emoji called `{name}`."),
                true,
            );
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        }
    };

    let emoji = components::emojis::emoji(ctx, emoji_id).await.unwrap();
    let emoji_name = &emoji.name;

    let result = match guild.delete_emoji(&ctx, emoji_id).await {
        Ok(_) => {
            let user_name = models::author_name(ctx)?;

            info!("@{user_name} deleted emoji called {emoji_name:?} from {guild_name}");
            Ok(format!("I've deleted an emoji called `{emoji_name}`."))
        }
        Err(why) => {
            error!("Couldn't delete {emoji_name:?} emoji from {guild_name}: {why:?}");
            Err(format!(
                "Sorry, but I couldn't delete an emoji called `{emoji_name}`"
            ))
        }
    };

    let reply = match result {
        Ok(message) => messages::ok_reply(message, true),
        Err(message) => messages::error_reply(message, true),
    };
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
