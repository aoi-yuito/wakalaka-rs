// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Guild, GuildId, ModelError};
use wakalaka_core::types::{Context, Throwable};

pub fn name_raw(ctx: &Context, guild_id: &GuildId) -> String {
    guild_id.name(ctx).unwrap_or_else(|| format!("{guild_id}"))
}

pub fn guild_cached_raw(ctx: &Context, guild_id: &GuildId) -> Throwable<Guild> {
    let guild = guild_id
        .to_guild_cached(ctx)
        .ok_or_else(|| Box::new(ModelError::GuildNotFound))
        .map(|guild| guild.clone())?;
    Ok(guild)
}

pub fn guild_cached(ctx: Context<'_>, guild_id: &GuildId) -> Throwable<Guild> {
    let guild = guild_id
        .to_guild_cached(&ctx)
        .ok_or_else(|| Box::new(ModelError::GuildNotFound))
        .map(|guild| guild.clone())?;
    Ok(guild)
}

pub fn guild(ctx: Context<'_>) -> Throwable<Guild> {
    let guild = ctx
        .guild()
        .ok_or_else(|| Box::new(ModelError::GuildNotFound))
        .map(|guild| guild.clone())?;
    Ok(guild)
}
