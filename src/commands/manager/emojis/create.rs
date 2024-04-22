// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Attachment, CreateAttachment};
use tracing::{error, info};
use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::{accessors, builders};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "CREATE_GUILD_EXPRESSIONS",
    required_bot_permissions = "SEND_MESSAGES | CREATE_GUILD_EXPRESSIONS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Create a new emoji.
pub(super) async fn create(
    ctx: Context<'_>,
    #[description = "Name to give."]
    #[min_length = 2]
    #[max_length = 32]
    name: String,
    #[description = "Image for representing."] image: Attachment,
) -> Throwable<()> {
    let emojis = accessors::emojis::gather_all_guild_emojis(ctx).await?;

    let emoji_count = emojis.len();
    if emoji_count >= 250 {
        let reply = builders::replies::build_error_reply_with_embed(
            "Cannot have more than `250` emojis.",
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let author = ctx.author();
    let author_name = &author.name;

    let img_url = &image.url;
    let img_wh = match image.dimensions() {
        Some(dims) => dims,
        None => {
            let reply = builders::replies::build_error_reply_with_embed(
                "Cannot use binary files as images.",
                true,
            );

            ctx.send(reply).await?;

            return Ok(());
        }
    };
    if img_wh.0 < 128 || img_wh.1 < 128 {
        let reply = builders::replies::build_error_reply_with_embed(
            "Cannot use images smaller than `128x128` pixels.",
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let att = CreateAttachment::url(ctx, &img_url).await?;
    let att_hash = att.to_base64();

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = &guild.name;

    let result = match guild.create_emoji(ctx, &name, &att_hash).await {
        Ok(_) => {
            info!("@{author_name} created :{name}: in {guild_name}");

            Ok(format!("{name:?} has been created."))
        }
        Err(e) => {
            error!("@{author_name} failed to create :{name}: in {guild_name}: {e:?}");

            Err(format!("An error occurred while creating {name:?}."))
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(msg) => builders::replies::build_error_reply_with_embed(msg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
