// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Guild;
use sqlx::SqlitePool;

use crate::{
    database::{checks, queries},
    Error, SContext,
};

pub(crate) async fn handle(
    ctx: &SContext,
    db: &SqlitePool,
    guild: &Guild,
    is_new: &Option<bool>,
) -> Result<(), Error> {
    if !is_new.is_some() {
        return Ok(());
    }

    let guild_id = guild.id;

    let guild_owner_id = guild.owner_id;
    let guild_owner = guild_owner_id.to_user(ctx).await?;

    let guild_restricted = checks::check_restricted_guild(ctx, db, guild, &guild_owner).await?;
    if guild_restricted {
        guild.leave(ctx).await?;
        return Ok(());
    }

    queries::users::insert_into(db, &guild_owner_id).await?;

    queries::guilds::insert_into(db, &guild_id, &guild_owner_id).await?;

    Ok(())
}
