// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Guild, UserId};
use sqlx::PgPool;

use wakalaka_core::{
    consts,
    types::{Context, SContext, Throwable},
};
use wakalaka_utils::builders::{messages, replies};

use super::queries;

pub async fn is_guild_restricted(ctx: &SContext, pool: &PgPool, guild: &Guild) -> Throwable<bool> {
    let guild_id = &guild.id;
    let guild_name = &guild.name;
    let guild_owner_id = guild.owner_id;
    let guild_owner = guild_owner_id.to_user(ctx).await?;

    if let Ok(_) = queries::restricted_guilds::fetch_guild_id_from_db(pool, guild_id).await {
        let message = messages::build_error_message_with_embed(format!(
            r#"Sorry, but {guild_name:?} can't invite yours truly into anymore.
                
                If you think this is a mistake, contact the [developer]({}) on [support server]({}).
                
                In the meantime, take a moment to think about what went down, because this is irreversible."#,
            consts::DEV_GITHUB_URL,
            consts::SERVER_INVITE_URL,
        ));

        guild_owner.dm(ctx, message).await?;

        return Ok(true);
    }

    Ok(false)
}

pub async fn is_user_restricted(
    pool: &PgPool,
    ctx: Context<'_>,
    user_id: &UserId,
) -> Throwable<bool> {
    if let Ok(_) = queries::restricted_users::fetch_user_id_from_db(pool, user_id).await {
        let reply = replies::build_error_reply_with_embed(
            format!(
                r#"Sorry, but you can't use yours truly anymore.
                
                If you think this is a mistake, contact the [developer]({}) on [support server]({}).
                
                In the meantime, take a moment to think about what went down, because this is irreversible."#,
                consts::DEV_GITHUB_URL,
                consts::SERVER_INVITE_URL,
            ),
            true,
        );

        ctx.send(reply).await?;

        return Ok(true);
    }

    Ok(false)
}
