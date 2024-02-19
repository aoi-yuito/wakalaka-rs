// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::{all::Attachment, builder::CreateAttachment};
use tracing::{error, info};

use crate::{
    utils::{components, models},
    Context, Error,
};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "CREATE_GUILD_EXPRESSIONS",
    required_bot_permissions = "MANAGE_GUILD | SEND_MESSAGES | CREATE_GUILD_EXPRESSIONS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Create a new emoji.
pub(super) async fn add(
    ctx: Context<'_>,
    #[description = "The name of the emoji."]
    #[min_length = 2]
    #[max_length = 32]
    name: String,
    #[description = "The image representing the emoji."] image: Attachment,
) -> Result<(), Error> {
    let image_width = match image.width {
        Some(width) => {
            if width < 128 {
                let reply = components::replies::error_reply_embed(
                    "Image width must be `128` pixels or more!",
                    true,
                );

                ctx.send(reply).await?;

                return Ok(());
            }
        }
        None => {
            let reply =
                components::replies::error_reply_embed("Attachment must be an image!", true);

            ctx.send(reply).await?;

            return Ok(());
        }
    };
    let image_height = match image.height {
        Some(height) => {
            if height < 128 {
                let reply = components::replies::error_reply_embed(
                    "Image height must be `128` pixels or more!",
                    true,
                );

                ctx.send(reply).await?;

                return Ok(());
            }
        }
        None => {
            let reply =
                components::replies::error_reply_embed("Attachment must be an image!", true);

            ctx.send(reply).await?;

            return Ok(());
        }
    };
    if image_width != image_height {
        let reply =
            components::replies::error_reply_embed("Image must be `128x128` in size!", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let image_url = &image.url;

    let attachment = CreateAttachment::url(ctx, &image_url).await?;
    let attachment_hash = attachment.to_base64();

    let guild = models::guilds::guild(ctx)?;
    let guild_name = &guild.name;

    let result = match guild.create_emoji(ctx, &name, &attachment_hash).await {
        Ok(_) => {
            let author = ctx.author();
            let author_name = &author.name;

            info!("@{author_name} created {name:?} in {guild_name}");
            Ok(format!("`{name}` has been created."))
        }
        Err(why) => {
            error!("Failed to create {name:?} in {guild_name}: {why:?}");
            Err(format!("An error occurred while creating `{name}`."))
        }
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
