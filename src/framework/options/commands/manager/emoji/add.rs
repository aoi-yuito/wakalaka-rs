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

use serenity::{all::Attachment, builder::CreateAttachment};
use tracing::{error, info, warn};

use crate::{
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Manager",
    required_permissions = "CREATE_GUILD_EXPRESSIONS",
    required_bot_permissions = "SEND_MESSAGES | CREATE_GUILD_EXPRESSIONS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Create a new emoji.
pub async fn add(
    ctx: Context<'_>,
    #[description = "The name of the emoji."]
    #[min_length = 2]
    #[max_length = 32]
    name: String,
    #[description = "The image used for the emoji."] image: Attachment,
) -> Result<(), Error> {
    let name_char_count = name.chars().count();
    if name_char_count < 2 || name_char_count > 32 {
        let reply = messages::info_reply(
            format!("Name of the emoji must be between `2` and `32` characters long."),
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let (image_width, image_height) = (
        match image.width {
            Some(width) => width,
            None => {
                warn!("Couldn't get width of image");
                return Ok(());
            }
        },
        match image.height {
            Some(height) => height,
            None => {
                warn!("Couldn't get height of image");
                return Ok(());
            }
        },
    );
    if image_width != 128 || image_height != 128 {
        let reply =
            messages::info_reply("Width and height of the image must be `128` pixels.", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let image_url = &image.url;

    let attachment = match CreateAttachment::url(ctx, &image_url).await {
        Ok(emoji) => emoji,
        Err(why) => {
            error!("Couldn't create emoji: {why:?}");
            return Err(why.into());
        }
    };
    let encoded_attachment = attachment.to_base64();

    let guild = models::guilds::guild(ctx)?;
    let guild_name = &guild.name;

    let result = match guild.create_emoji(ctx, &name, &encoded_attachment).await {
        Ok(_) => {
            let user_name = models::users::author_name(ctx)?;

            info!("@{user_name} created emoji called {name:?} in {guild_name}");
            Ok(format!("I've created an emoji called `{name}`."))
        }
        Err(why) => {
            error!("Couldn't create emoji called{name:?} in {guild_name}: {why:?}");
            Err(format!(
                "Sorry, but I couldn't create an emoji called `{name}`."
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
