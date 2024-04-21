// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateEmbedAuthor, CreateEmbedFooter};
use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::builders;

#[poise::command(
    slash_command,
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5,
    ephemeral
)]
/// Get information about yours truly.
pub(super) async fn about(ctx: Context<'_>) -> Throwable<()> {
    let author = ctx.author();
    let author_face = author.face();

    let embed_author = CreateEmbedAuthor::new(crate::CARGO_AUTHORS).icon_url(author_face);
    let embed_footer = CreateEmbedFooter::new(crate::CARGO_VERSION);
    let embed = builders::embeds::build_embed(Some(format!("{}", crate::CARGO_DESCRIPTION)))
        .author(embed_author)
        .title(crate::CARGO_NAME)
        .url(crate::CARGO_REPOSITORY)
        .image(crate::RES_MASCOT_IMAGE_URL.as_str())
        .footer(embed_footer);

    let reply = builders::replies::build_reply_with_optional_embed("", &Some(embed), true);

    ctx.send(reply).await?;

    Ok(())
}
