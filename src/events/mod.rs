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

use serenity::all::FullEvent;
use wakalaka_core::{
    types::{FrameworkContext, SContext, Throwable},
    Data,
};

pub async fn handle_event_handler_option(
    ctx: &SContext,
    event: &FullEvent,
    _fw_ctx: FrameworkContext<'_>,
    data: &Data,
) -> Throwable<()> {
    let db = &data.db;

    match event {
        FullEvent::CategoryCreate { category, .. } => {
            category::category_create::handle_category_create_event(ctx, category).await?
        }
        FullEvent::CategoryDelete { category, .. } => {
            category::category_delete::handle_category_delete_event(ctx, category).await?
        }
        FullEvent::ChannelCreate { channel, .. } => {
            channel::channel_create::handle_channel_create_event(ctx, channel).await?
        }
        FullEvent::ChannelDelete { channel, .. } => {
            channel::channel_delete::handle_channel_delete_event(ctx, channel).await?
        }
        FullEvent::GuildBanAddition {
            guild_id,
            banned_user,
        } => {
            guild::guild_ban_addition::handle_guild_ban_addition_event(ctx, guild_id, banned_user)
                .await?;
        }
        FullEvent::GuildBanRemoval {
            guild_id,
            unbanned_user,
        } => {
            guild::guild_ban_removal::handle_guild_ban_removal_event(ctx, guild_id, unbanned_user)
                .await?;
        }
        FullEvent::GuildMemberAddition { new_member } => {
            guild::guild_member_addition::handle_guild_member_addition_event(ctx, new_member, db)
                .await?;
        }
        FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
            guild::guild_member_removal::handle_guild_member_removal_event(ctx, guild_id, user, db)
                .await?;
        }
        FullEvent::GuildRoleCreate { new } => {
            guild::guild_role_create::handle_guild_role_create_event(ctx, new).await?
        }
        FullEvent::GuildRoleDelete {
            guild_id,
            removed_role_id,
            ..
        } => {
            guild::guild_role_delete::handle_guild_role_delete_event(ctx, guild_id, removed_role_id)
                .await?
        }
        FullEvent::GuildCreate { guild, is_new } => {
            guild::guild_create::handle_guild_create_event(ctx, guild, is_new, db).await?
        }
        FullEvent::GuildDelete { incomplete, full } => {
            guild::guild_delete::handle_guild_delete_event(ctx, incomplete, full, db).await?
        }
        FullEvent::GuildUpdate { new_data, .. } => {
            guild::guild_update::handle_guild_update_event(new_data, db).await?
        }
        FullEvent::InviteCreate { data } => {
            invite::invite_create::handle_invite_create_event(ctx, data).await?
        }
        FullEvent::InviteDelete { data } => {
            invite::invite_delete::handle_invite_delete_event(ctx, data).await?
        }
        FullEvent::CacheReady { guilds } => {
            ready::cache_ready::handle_cache_ready_event(ctx, guilds).await?
        }
        FullEvent::Ready { data_about_bot } => {
            ready::handle_ready_event(ctx, data_about_bot).await?
        }
        FullEvent::ShardsReady { total_shards } => {
            ready::shards_ready::handle_shards_ready_event(total_shards).await?
        }
        FullEvent::ThreadCreate { thread, .. } => {
            thread::thread_create::handle_thread_create_event(ctx, thread).await?
        }
        FullEvent::ThreadDelete {
            full_thread_data, ..
        } => thread::thread_delete::handle_thread_delete_event(ctx, full_thread_data).await?,
        _ => {}
    }

    Ok(())
}
