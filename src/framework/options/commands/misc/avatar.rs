// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use serenity::{
    all::{colours::branding, CreateEmbedFooter, User},
    builder::CreateEmbed,
};

use crate::{Context, Throwable};

#[poise::command(
    slash_command,
    context_menu_command = "Avatar",
    category = "Miscellaneous",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5
)]
/// Get a user's avatar.
pub(super) async fn avatar(
    ctx: Context<'_>,
    #[description = "User to get an avatar from."] user: User,
) -> Throwable<()> {
    let user_id = user.id;

    let raw_user = ctx.http().get_user(user_id).await?;

    let user_name = &raw_user.name;
    let user_face = raw_user.face();

    let user_accent_colour = match raw_user.accent_colour {
        Some(colour) => colour,
        None => branding::BLURPLE,
    };

    let embed_footer = CreateEmbedFooter::new(format!("🆔{user_id}"));

    let embed = CreateEmbed::default()
        .title(user_name)
        .image(user_face)
        .footer(embed_footer)
        .colour(user_accent_colour);

    let reply = CreateReply::default().embed(embed);

    ctx.send(reply).await?;

    Ok(())
}
