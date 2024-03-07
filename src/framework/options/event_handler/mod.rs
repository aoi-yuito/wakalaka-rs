// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod category;
mod channel;
mod guild;
mod invite;
mod ready;
mod thread;

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
        FullEvent::CacheReady { guilds, .. } => ready::cache_ready::handle(guilds).await?,
        FullEvent::ShardsReady { total_shards, .. } => {
            ready::shards_ready::handle(total_shards).await?
        }
        FullEvent::ChannelCreate { channel, .. } => {
            channel::channel_create::handle(ctx, channel).await?
        }
        FullEvent::CategoryCreate { category, .. } => {
            category::category_create::handle(ctx, category).await?
        }
        FullEvent::CategoryDelete { category, .. } => {
            category::category_delete::handle(ctx, category).await?
        }
        FullEvent::ChannelDelete { channel, .. } => {
            channel::channel_delete::handle(ctx, channel).await?
        }
        FullEvent::GuildBanAddition {
            guild_id,
            banned_user,
        } => guild::ban::guild_ban_addition::handle(ctx, guild_id, banned_user).await?,
        FullEvent::GuildBanRemoval {
            guild_id,
            unbanned_user,
        } => guild::ban::guild_ban_removal::handle(ctx, guild_id, unbanned_user).await?,
        FullEvent::GuildCreate { guild, is_new, .. } => {
            guild::guild_create::handle(ctx, db, guild, is_new).await?
        }
        FullEvent::GuildDelete {
            incomplete, full, ..
        } => guild::guild_delete::handle(ctx, db, incomplete, full).await?,
        FullEvent::GuildMemberAddition { new_member, .. } => {
            guild::member::guild_member_addition::handle(ctx, db, new_member).await?
        }
        FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
            guild::member::guild_member_removal::handle(ctx, guild_id, user).await?
        }
        FullEvent::GuildRoleCreate { new } => {
            guild::role::guild_role_create::handle(ctx, new).await?
        }
        FullEvent::GuildRoleDelete {
            guild_id,
            removed_role_id,
            ..
        } => guild::role::guild_role_delete::handle(ctx, guild_id, removed_role_id).await?,
        FullEvent::GuildUpdate { new_data, .. } => {
            guild::guild_update::handle(db, new_data).await?
        }
        FullEvent::InviteCreate { data, .. } => invite::invite_create::handle(ctx, data).await?,
        FullEvent::InviteDelete { data, .. } => invite::invite_delete::handle(ctx, data).await?,
        FullEvent::Ready { data_about_bot, .. } => ready::handle(ctx, data_about_bot).await?,
        FullEvent::ThreadCreate { thread, .. } => {
            thread::thread_create::handle(ctx, thread).await?
        }
        FullEvent::ThreadDelete {
            thread: _,
            full_thread_data,
        } => thread::thread_delete::handle(ctx, full_thread_data).await?,
        _ => {}
    }

    Ok(())
}
