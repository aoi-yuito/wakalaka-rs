// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Member;
use sqlx::SqlitePool;
use tracing::info;

use crate::{database::queries, utils::models, SContext, Throwable};

pub(crate) async fn handle(ctx: &SContext, db: &SqlitePool, member: &Member) -> Throwable<()> {
    if member.user.bot || member.user.system {
        return Ok(());
    }

    let member_id = &member.user.id;
    let member_name = &member.user.name;

    let guild_id = member.guild_id;
    let guild_name = models::guilds::name_raw(ctx, &guild_id);

    info!("@{member_name} joined {guild_name}");

    queries::users::insert_into(db, &member_id).await?;

    Ok(())
}
