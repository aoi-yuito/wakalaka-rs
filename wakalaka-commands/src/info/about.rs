// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateEmbedAuthor, CreateEmbedFooter};

use wakalaka_core::{
    builders, consts,
    types::{Context, Throwable},
};

#[poise::command(
    slash_command,
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5,
    ephemeral
)]
/// Get information about yours truly.
pub(super) async fn about(ctx: Context<'_>) -> Throwable<()> {
    let http = ctx.http();

    let bot = http.get_current_user().await?;
    let bot_name = &bot.name;
    let bot_face = bot.face();

    let author = ctx.author();
    let author_name = &author.name;
    let author_face = author.face();

    let embed_author = CreateEmbedAuthor::new(format!("{bot_name} {}", consts::CARGO_VERSION))
        .icon_url(bot_face)
        .url(consts::CARGO_REPOSITORY);
    let embed_footer = CreateEmbedFooter::new(author_name).icon_url(author_face);
    let embed = builders::embeds::build_embed(Some(format!("{}", consts::CARGO_DESCRIPTION)))
        .author(embed_author)
        .image(consts::RES_MASCOT_IMAGE_URL.as_str())
        .footer(embed_footer);

    let reply = builders::replies::build_reply(None::<String>, &Some(embed), true);

    ctx.send(reply).await?;

    Ok(())
}