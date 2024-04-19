// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Emoji, EmojiId};
use wakalaka_core::types::{Context, Throwable};

use super::guilds;

pub async fn gather_all_guild_emojis(ctx: Context<'_>) -> Throwable<Vec<Emoji>> {
    let guild = guilds::fetch_guild(ctx)?;

    let emojis = guild.emojis(ctx).await?;
    Ok(emojis)
}

pub async fn fetch_guild_emoji(ctx: Context<'_>, emoji_id: &EmojiId) -> Throwable<Emoji> {
    let guild = guilds::fetch_guild(ctx)?;

    let emoji = guild.emoji(ctx, *emoji_id).await?;
    Ok(emoji)
}
