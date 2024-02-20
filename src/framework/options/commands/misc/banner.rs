// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use serenity::{
    all::{colours::branding, Mentionable, User},
    builder::CreateEmbed,
};

use crate::{utils::components, Context, Error};

#[poise::command(
    slash_command,
    context_menu_command = "Banner",
    category = "Miscellaneous",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5
)]
/// Get a user's banner.
pub(super) async fn banner(
    ctx: Context<'_>,
    #[description = "The user to get the banner of."] user: User,
) -> Result<(), Error> {
    let user_id = user.id;
    let user = ctx.http().get_user(user_id).await?;
    let user_name = &user.name;
    let user_mention = user.mention();

    let user_banner_url = user.banner_url();
    if let Some(user_banner_url) = user_banner_url {
        let user_accent_colour = user.accent_colour.unwrap_or(branding::BLURPLE);

        let embed = CreateEmbed::default()
            .title(user_name)
            .image(user_banner_url)
            .colour(user_accent_colour);

        let reply = CreateReply::default().embed(embed);

        ctx.send(reply).await?;

        return Ok(());
    }

    let reply = components::replies::error_reply_embed(
        format!("{user_mention} does not have a banner!"),
        true,
    );

    ctx.send(reply).await?;

    Ok(())
}
