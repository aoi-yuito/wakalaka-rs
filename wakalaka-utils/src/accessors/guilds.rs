// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Guild, GuildId, ModelError};
use wakalaka_core::types::{Context, SContext, Throwable};

pub fn fetch_raw_guild_name(ctx: &SContext, guild_id: &GuildId) -> String {
    guild_id.name(ctx).unwrap_or_else(|| format!("{guild_id}"))
}

pub fn fetch_raw_cached_guild(ctx: &SContext, guild_id: &GuildId) -> Throwable<Guild> {
    let guild = guild_id
        .to_guild_cached(ctx)
        .ok_or_else(|| Box::new(ModelError::GuildNotFound))
        .map(|guild| guild.clone())?;
    Ok(guild)
}

pub fn fetch_cached_guild(ctx: Context<'_>, guild_id: &GuildId) -> Throwable<Guild> {
    let guild = guild_id
        .to_guild_cached(&ctx)
        .ok_or_else(|| Box::new(ModelError::GuildNotFound))
        .map(|guild| guild.clone())?;
    Ok(guild)
}

pub fn fetch_guild(ctx: Context<'_>) -> Throwable<Guild> {
    let guild = ctx
        .guild()
        .ok_or_else(|| Box::new(ModelError::GuildNotFound))
        .map(|guild| guild.clone())?;
    Ok(guild)
}
