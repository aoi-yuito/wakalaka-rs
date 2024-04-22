// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Emoji;
use wakalaka_core::types::{Context, Throwable};

use super::guilds;

pub async fn gather_all_guild_emojis(ctx: Context<'_>) -> Throwable<Vec<Emoji>> {
    let guild = guilds::fetch_guild(ctx)?;

    let emojis = guild.emojis(ctx).await?;
    Ok(emojis)
}
