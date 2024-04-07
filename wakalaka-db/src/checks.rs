// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Guild, UserId};
use sqlx::SqlitePool;
use wakalaka_core::{
    consts::{GITHUB_URL, INVITE_URL},
    types::{Context, SContext, Throwable},
};
use wakalaka_utils::factories::{messages, replies};

use crate::queries::{restricted_guilds, restricted_users};

pub async fn is_guild_restricted(
    ctx: &SContext,
    pool: &SqlitePool,
    guild: &Guild,
) -> Throwable<bool> {
    let guild_id = &guild.id;
    let guild_name = &guild.name;
    let guild_owner_id = guild.owner_id;
    let guild_owner = guild_owner_id.to_user(ctx).await?;

    if let Ok(_) = restricted_guilds::fetch_guild_id_from_db(pool, guild_id).await {
        let message = messages::build_error_message_with_embed(format!(
            r#"Sorry, but {guild_name} can't invite yours truly into anymore.
                
                If you think this is a mistake, contact the [developer]({GITHUB_URL}) on [support server]({INVITE_URL}).
                
                In the meantime, take a moment to think about what went down, because this is irreversible"#
        ));

        guild_owner.dm(ctx, message).await?;

        guild.leave(ctx).await?;

        return Ok(true);
    }

    Ok(false)
}

pub async fn is_user_restricted(
    pool: &SqlitePool,
    ctx: Context<'_>,
    user_id: &UserId,
) -> Throwable<bool> {
    if let Ok(_) = restricted_users::fetch_user_id_from_db(pool, user_id).await {
        let reply = replies::build_error_reply_with_embed(
            format!(
                r#"Sorry, but you can't use yours truly anymore.
                
                If you think this is a mistake, contact the [developer]({GITHUB_URL}) on [support server]({INVITE_URL}).
                
                In the meantime, take a moment to think about what went down, because this is irreversible"#
            ),
            true,
        );

        ctx.send(reply).await?;

        return Ok(true);
    }

    Ok(false)
}
