// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Guild, User};
use sqlx::SqlitePool;

use crate::{
    utils::builders,
    utils::{GITHUB_URL, INVITE_URL},
    Context, SContext, Throwable,
};

use super::queries;

pub(crate) async fn check_restricted_guild(
    ctx: &SContext,
    db: &SqlitePool,
    guild: &Guild,
    owner: &User,
) -> Throwable<bool> {
    let guild_id = guild.id;
    let guild_name = &guild.name;

    if let Err(_) = queries::restricted_guilds::select_guild_id(db, &guild_id).await {
        return Ok(false);
    }

    let message = builders::messages::error_message_embed(format!(
        "Sorry, but you can't have yours truly in {guild_name} anymore.\n\nIf you think this is a mistake, contact the [developer]({GITHUB_URL}) on the [support server]({INVITE_URL}).\n\nIn the meantime, take a moment to think about what went down, because this is irreversible."));

    owner.dm(ctx, message).await?;

    guild.leave(ctx).await?;

    Ok(true)
}

pub(crate) async fn check_restricted_user(
    ctx: Context<'_>,
    db: &SqlitePool,
    user: &User,
) -> Throwable<bool> {
    let user_id = user.id;

    if let Err(_) = queries::restricted_users::select_user_id(db, &user_id).await {
        return Ok(false);
    }

    let reply = builders::replies::error_reply_embed(format!(
        "Sorry, but you can't use yours truly anymore.\n\nIf you think this is a mistake, contact the [developer]({GITHUB_URL}) on the [support server]({INVITE_URL}).\n\nIn the meantime, take a moment to think about what went down, because this is irreversible."), true);

    ctx.send(reply).await?;

    Ok(true)
}
