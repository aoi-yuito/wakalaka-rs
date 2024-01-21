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
/// Create a new emoji.
pub(crate) async fn addemoji(
    ctx: Context<'_>,
    #[description = "The name of the emoji."] name: String,
    #[description = "The image to use for the emoji. (128x128)"] image: Attachment,
) -> Result<(), Error> {
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
        let reply = messages::warn_reply("Image must be `128`x`128` pixels in size.", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(Error::from(why));
        }

        return Ok(());
    }

    let image_url = &image.url;

    let attachment = match CreateAttachment::url(ctx, &image_url).await {
        Ok(emoji) => emoji,
        Err(why) => {
            error!("Couldn't create emoji: {why:?}");
            return Err(Error::from(why));
        }
    };
    let encoded_attachment = attachment.to_base64();

    let guild = utility::guilds::guild(ctx).await;
    let guild_name = &guild.name;

    if let Err(why) = guild.create_emoji(&ctx, &name, &encoded_attachment).await {
        error!("Couldn't create {name:?} emoji in {guild_name}: {why:?}");

        let reply =
            messages::error_reply(format!("Couldn't create an emoji called `{name}`."), true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(Error::from(why));
        }

        return Err(Error::from(why));
    }

    info!("Created {name:?} emoji in {guild_name}");

    let reply = messages::ok_reply(format!("Created an emoji called `{name}`."), true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(Error::from(why));
    }

    Ok(())
}
