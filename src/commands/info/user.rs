// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{colours::branding, CreateEmbedFooter, User};
use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::builders;

#[poise::command(
    slash_command,
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5
)]
/// Get information about a user.
pub(super) async fn user(
    ctx: Context<'_>,
    #[description = "User to get information about, if any."] user: Option<User>,
) -> Throwable<()> {
    let author = ctx.author();

    let user = user.unwrap_or(author.clone());
    let user_id = user.id;
    let user_name = &user.name;
    let user_face = user.face();
    let user_created_at = user.created_at();

    let http = ctx.http();

    let raw_user = http.get_user(user_id).await?;
    let raw_user_banner = raw_user.banner_url().unwrap_or_default();
    let raw_user_accent_col = raw_user.accent_colour.unwrap_or(branding::BLURPLE);

    let embed_footer = CreateEmbedFooter::new(format!("🆔{user_id}"));
    let embed = builders::embeds::build_embed(None)
        .title(format!("{user_name}"))
        .thumbnail(user_face)
        .image(raw_user_banner)
        .colour(raw_user_accent_col)
        .footer(embed_footer)
        .timestamp(user_created_at);

    let reply = builders::replies::build_reply_with_optional_embed("", &Some(embed), true);

    ctx.send(reply).await?;

    Ok(())
}
