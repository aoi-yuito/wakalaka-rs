// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Mentionable;
use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::builders;

#[poise::command(
    slash_command,
    category = "Fun",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Flip a coin.
pub(super) async fn flip(ctx: Context<'_>) -> Throwable<()> {
    let flipped = rand::random::<bool>();

    let author = ctx.author();
    let author_mention = author.mention();

    let flip_result = if flipped { "heads" } else { "tails" };

    let reply = builders::replies::build_reply_with_embed(
        format!(":coin: {author_mention} flipped a coin and got `{flip_result}`!"),
        false,
    );

    ctx.send(reply).await?;

    Ok(())
}
