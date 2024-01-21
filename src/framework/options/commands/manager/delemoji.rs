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
    utility::{self, components::messages},
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
pub(crate) async fn delemoji(
    ctx: Context<'_>,
    #[description = "The name of the emoji."] name: String,
) -> Result<(), Error> {
    let guild = utility::guilds::guild(ctx).await;
    let guild_name = &guild.name;

    let emoji_id = match utility::components::emojis::emoji_id(ctx, &name).await {
        Some(emoji_id) => emoji_id,
        None => {
            warn!("Couldn't find {name:?} emoji in {guild_name}");

            let reply =
                messages::error_reply(format!("Couldn't find an emoji called `{name}`."), true);
            if let Err(why) = ctx.send(reply).await {
                error!("Couldn't send reply: {why:?}");
                return Err(Error::from(why));
            }

            return Ok(());
        }
    };

    let emoji = utility::components::emojis::emoji(ctx, emoji_id)
        .await
        .unwrap();
    let emoji_name = &emoji.name;

    if let Err(why) = guild.delete_emoji(&ctx, emoji_id).await {
        error!("Couldn't delete {emoji_name:?} emoji from {guild_name}: {why:?}");

        let reply = messages::error_reply(
            format!("Couldn't delete an emoji called `{emoji_name}`"),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(Error::from(why));
        }

        return Err(Error::from(why));
    }

    info!("Deleted {emoji_name:?} emoji from {guild_name}");

    let reply = messages::ok_reply(format!("Deleted an emoji called `{emoji_name}`."), true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(Error::from(why));
    }

    Ok(())
}
