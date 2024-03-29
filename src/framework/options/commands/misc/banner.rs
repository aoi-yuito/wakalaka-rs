// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use serenity::{
    all::{CreateEmbedFooter, Mentionable, User},
    builder::CreateEmbed,
};

use crate::{utils::builders, Context, Throwable};

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
    #[description = "User to get a banner from."] user: User,
) -> Throwable<()> {
    let user_id = user.id;

    let raw_user = ctx.http().get_user(user_id).await?;

    let user_name = &raw_user.name;
    let user_mention = raw_user.mention();

    let embed_footer = CreateEmbedFooter::new(format!("🆔{user_id}"));

    let user_banner_url = raw_user.banner_url();
    if let Some(user_banner_url) = user_banner_url {
        let user_accent_colour = raw_user.accent_colour.unwrap_or_default();

        let embed = CreateEmbed::default()
            .title(user_name)
            .image(user_banner_url)
            .footer(embed_footer)
            .colour(user_accent_colour);

        let reply = CreateReply::default().embed(embed);

        ctx.send(reply).await?;

        return Ok(());
    }

    let reply = builders::replies::error_reply_embed(
        format!("{user_mention} does not have a banner!"),
        true,
    );

    ctx.send(reply).await?;

    Ok(())
}
