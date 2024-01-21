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
    utility::{
        self,
        components::{self, messages},
    },
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "CREATE_GUILD_EXPRESSIONS",
    guild_only,
    ephemeral
)]
/// Delete an existing emoji.
pub(crate) async fn delete(
    ctx: Context<'_>,
    #[description = "The name of the emoji."]
    #[min_length = 2]
    #[max_length = 32]
    name: String,
) -> Result<(), Error> {
    let number_of_name = name.chars().count();
    if number_of_name < 2 || number_of_name > 32 {
        let reply = messages::warn_reply(
            format!("Emoji name must be between 2 and 32 characters."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let guild = utility::guilds::guild(ctx).await;
    let guild_name = &guild.name;

    let emoji_id = match components::emojis::emoji_id(ctx, &name).await {
        Some(emoji_id) => emoji_id,
        None => {
            warn!("Couldn't find {name:?} emoji in {guild_name}");

            let reply =
                messages::error_reply(format!("Couldn't find an emoji called `{name}`."), true);
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        }
    };

    let emoji = components::emojis::emoji(ctx, emoji_id).await.unwrap();
    let emoji_name = &emoji.name;

    if let Err(why) = guild.delete_emoji(&ctx, emoji_id).await {
        error!("Couldn't delete {emoji_name:?} emoji from {guild_name}: {why:?}");

        let reply = messages::error_reply(
            format!("Couldn't delete an emoji called `{emoji_name}`"),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Err(why.into());
    }

    info!("Deleted {emoji_name:?} emoji from {guild_name}");

    let reply = messages::ok_reply(format!("Deleted an emoji called `{emoji_name}`."), true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
