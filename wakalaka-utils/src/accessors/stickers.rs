// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Sticker;
use wakalaka_core::types::{Context, Throwable};

use super::guilds;

pub async fn gather_all_guild_stickers(ctx: Context<'_>) -> Throwable<Vec<Sticker>> {
    let guild = guilds::fetch_guild(ctx)?;
    let guild_id = guild.id;

    let stickers = guild_id.stickers(ctx).await?;
    Ok(stickers)
}
