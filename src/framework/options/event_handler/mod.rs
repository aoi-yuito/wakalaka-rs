// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod guild;
mod ready;

use serenity::client::FullEvent;

use crate::{Data, Error, FrameworkContext, SContext};

pub(crate) async fn handle(
    ctx: &SContext,
    event: &FullEvent,
    _framework_ctx: FrameworkContext<'_>,
    data: &Data,
) -> Result<(), Error> {
    let db = &data.db;

    match event {
        FullEvent::GuildCreate { guild, is_new, .. } => {
            guild::guild_create::handle(ctx, db, guild, is_new).await?
        }
        FullEvent::GuildDelete {
            incomplete, full, ..
        } => guild::guild_delete::handle(ctx, db, incomplete, full).await?,
        FullEvent::GuildMemberAddition { new_member, .. } => {
            guild::guild_member_addition::handle(ctx, db, new_member).await?
        }
        FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
            guild::guild_member_removal::handle(ctx, guild_id, user).await?
        }
        FullEvent::GuildUpdate { new_data, .. } => {
            guild::guild_update::handle(db, new_data).await?
        }
        FullEvent::Ready { data_about_bot, .. } => ready::handle(ctx, data_about_bot).await?,
        _ => {}
    }

    Ok(())
}
