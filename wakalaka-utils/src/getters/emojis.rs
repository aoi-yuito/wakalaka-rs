// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Emoji, EmojiId};
use wakalaka_core::types::{Context, Throwable};

use super::guilds;

pub async fn emojis(ctx: Context<'_>) -> Throwable<Vec<Emoji>> {
    let guild = guilds::guild(ctx)?;

    let emojis = guild.emojis(ctx).await?;
    Ok(emojis)
}

pub async fn emoji(ctx: Context<'_>, emoji_id: &EmojiId) -> Throwable<Emoji> {
    let guild = guilds::guild(ctx)?;

    let emoji = guild.emoji(ctx, *emoji_id).await?;
    Ok(emoji)
}
