// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use serenity::{
    all::{colours::branding, User},
    builder::CreateEmbed,
};

use crate::{Context, Error};

#[poise::command(
    slash_command,
    context_menu_command = "Get Avatar",
    category = "Miscellaneous",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5
)]
/// Get a user's avatar.
pub(super) async fn avatar(
    ctx: Context<'_>,
    #[description = "The user to get avatar of"] user: User,
) -> Result<(), Error> {
    let user_name = &user.name;
    let user_face = user.face();

    let user_accent_colour = user.accent_colour.unwrap_or(branding::BLURPLE);

    let embed = CreateEmbed::default()
        .title(user_name)
        .image(user_face)
        .colour(user_accent_colour);

    let reply = CreateReply::default().embed(embed);

    ctx.send(reply).await?;

    Ok(())
}
