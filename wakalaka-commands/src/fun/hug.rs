// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateEmbedFooter, Mentionable, User};

use wakalaka_core::{
    builders,
    types::{Context, Throwable},
};
use wakalaka_integrations::kawaii;

#[poise::command(
    slash_command,
    context_menu_command = "Hug",
    category = "Fun",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Hug a user.
pub(super) async fn hug(
    ctx: Context<'_>,
    #[description = "User to hug."]
    user: User,
) -> Throwable<()> {
    let author = ctx.author();
    let author_id = author.id;

    let user_id = user.id;
    let user_mention = user.mention();

    let result = if author_id == user_id {
        Err(format!("Cannot hug yourself."))
    } else {
        Ok(format!(":people_hugging: {user_mention}"))
    };
    match result {
        Ok(msg) => {
            let hug_gif_url =
                kawaii::gif::fetch_gif_by_endpoint(kawaii::gif::Endpoint::Hug).await?;

            let embed_footer =
                CreateEmbedFooter::new("Powered by Kawaii API").icon_url(kawaii::KAWAII_LOGO_URL);
            let embed = builders::embeds::build_embed(None)
                .image(hug_gif_url)
                .footer(embed_footer);

            let reply = builders::replies::build_reply(Some(msg), &Some(embed), false);

            ctx.send(reply).await?;
        }
        Err(msg) => {
            let reply = builders::replies::build_error_reply_with_embed(msg, true);

            ctx.send(reply).await?;
        }
    };

    Ok(())
}
