// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use serenity::builder::{CreateEmbedAuthor, CreateEmbedFooter};

use crate::{
    utils::{
        components, CARGO_AUTHORS, CARGO_DESCRIPTION, CARGO_NAME, CARGO_REPOSITORY, CARGO_VERSION,
    },
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5,
    ephemeral
)]
/// Get information about yours truly.
pub(super) async fn about(ctx: Context<'_>) -> Throwable<()> {
    let embed_author = CreateEmbedAuthor::new(CARGO_AUTHORS);
    let embed_footer = CreateEmbedFooter::new(CARGO_VERSION);

    let embed = components::embeds::embed(CARGO_DESCRIPTION)
        .author(embed_author)
        .title(CARGO_NAME)
        .url(CARGO_REPOSITORY)
        .footer(embed_footer);

    let reply = CreateReply::default().embed(embed);

    ctx.send(reply).await?;

    Ok(())
}
