// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Mentionable;

use crate::{utils::builders, Context, Throwable};

#[poise::command(
    slash_command,
    category = "Fun",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Flip a coin.
pub(super) async fn flip(ctx: Context<'_>) -> Throwable<()> {
    let random = rand::random::<bool>();

    let author = ctx.author();
    let author_mention = author.mention();

    let reply = if random {
        builders::replies::reply_embed(
            format!("{author_mention} flipped a coin and got **heads**!"),
            false,
        )
    } else {
        builders::replies::reply_embed(
            format!("{author_mention} flipped a coin and got **tails**!"),
            false,
        )
    };

    ctx.send(reply).await?;

    Ok(())
}
