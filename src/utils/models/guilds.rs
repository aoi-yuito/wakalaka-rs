// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::{
    all::{Guild, GuildId},
    model::ModelError,
};
use tracing::warn;

use crate::{Context, SContext, Throwable};

pub(crate) fn name_raw(ctx: &SContext, guild_id: &GuildId) -> String {
    match guild_id.name(ctx) {
        Some(name) => name,
        None => {
            warn!("No name from ID of guild found, using ID as name");
            format!("{guild_id}")
        }
    }
}

pub(crate) fn guild_from_id_raw(ctx: &SContext, guild_id: &GuildId) -> Throwable<Guild> {
    match guild_id.to_guild_cached(ctx) {
        Some(guild) => Ok(guild.clone()),
        None => Err(Box::new(ModelError::GuildNotFound)),
    }
}

pub(crate) fn guild_from_id(ctx: Context<'_>, guild_id: &GuildId) -> Throwable<Guild> {
    match guild_id.to_guild_cached(&ctx) {
        Some(guild) => Ok(guild.clone()),
        None => Err(Box::new(ModelError::GuildNotFound)),
    }
}

pub(crate) fn guild(ctx: Context<'_>) -> Throwable<Guild> {
    match ctx.guild() {
        Some(guild) => Ok(guild.clone()),
        None => Err(Box::new(ModelError::GuildNotFound)),
    }
}
