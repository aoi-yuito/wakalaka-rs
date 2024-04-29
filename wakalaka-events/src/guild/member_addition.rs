// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Member;
use sqlx::PgPool;

use wakalaka_core::{
    accessors,
    types::{SContext, Throwable},
};
use wakalaka_database::queries;

pub(crate) async fn handle_guild_member_addition_event(
    ctx: &SContext,
    member: &Member,
    pool: &PgPool,
) -> Throwable<()> {
    let user = &member.user;
    if user.bot || user.system {
        return Ok(());
    }

    let user_id = &user.id;
    let user_name = &user.name;
    let user_created_at = &user.created_at();

    let guild_id = &member.guild_id;
    let guild = accessors::guilds::fetch_raw_cached_guild(ctx, guild_id)?;
    let guild_name = guild.name;

    tracing::info!("@{user_name} joined {guild_name}");

    queries::users::add_user_to_db(pool, user_id, user_created_at).await?;

    Ok(())
}
