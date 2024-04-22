// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Attachment, CreateAttachment, CreateSticker, PremiumTier};

use wakalaka_core::{
    consts,
    types::{Context, Throwable},
};
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
/// Create a new sticker.
pub(super) async fn create(
    ctx: Context<'_>,
    #[description = "Name to give."]
    #[min_length = 2]
    #[max_length = 30]
    name: String,
    #[description = "Description to give, if any."]
    #[min_length = 2]
    #[max_length = 100]
    description: Option<String>,
    #[description = "Image for representing."] file: Attachment,
) -> Throwable<()> {
    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = &guild.name;
    let guild_premium_tier = guild.premium_tier;

    let stickers = accessors::stickers::gather_all_guild_stickers(ctx).await?;

    let sticker_count = stickers.len();
    let max_sticker_count = match guild_premium_tier {
        PremiumTier::Tier0 => 5,
        PremiumTier::Tier1 => 15, // LVL 1
        PremiumTier::Tier2 => 30, // LVL 2
        PremiumTier::Tier3 => 60, // LVL 3
        _ => 5,
    };
    if sticker_count >= max_sticker_count {
        let reply = builders::replies::build_error_reply_with_embed(
            format!("Cannot have more than `{max_sticker_count}` stickers."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let author = ctx.author();
    let author_name = &author.name;

    match &file.dimensions() {
        Some(dims) => {
            if dims.0 > consts::STICKER_MAX_DIMENSIONS.0
                || dims.1 > consts::STICKER_MAX_DIMENSIONS.1
            {
                let reply = builders::replies::build_warning_reply_with_embed(
                    format!("`{}x{}` is too large for a sticker.", dims.0, dims.1),
                    true,
                );

                ctx.send(reply).await?;

                return Ok(());
            }
        }
        None => {
            let reply = builders::replies::build_error_reply_with_embed(
                "An error occurred while fetching image dimensions.",
                true,
            );

            ctx.send(reply).await?;

            return Ok(());
        }
    };

    let img_url = &file.url;
    let img_ext = img_url.split('.').last().unwrap_or_default();
    let img_size = &file.size;
    if !consts::STICKER_EXTENSIONS.contains(&img_ext) {
        let reply = builders::replies::build_warning_reply_with_embed(
            format!("`{img_ext}` is not a valid extension for a sticker."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }
    if img_size > &consts::STICKER_MAX_SIZE {
        let reply = builders::replies::build_warning_reply_with_embed(
            format!("`{img_size}` is too large for a sticker."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let att = CreateAttachment::url(ctx, &img_url).await?;

    let sticker = CreateSticker::new(&name, att).description(description.unwrap_or_default());

    let result = match guild.create_sticker(ctx, sticker).await {
        Ok(_) => {
            tracing::info!("@{author_name} created {name:?} in {guild_name}");

            Ok(format!("`{name}` has been created."))
        }
        Err(e) => {
            tracing::error!("@{author_name} failed to create {name:?} in {guild_name}: {e:?}");

            Err(format!("An error occurred while creating `{name}`."))
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(msg) => builders::replies::build_error_reply_with_embed(msg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
