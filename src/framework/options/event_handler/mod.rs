// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

mod cache_ready;
mod channel;
mod guild;
mod interaction;
mod message;
mod ready;

use poise::serenity_prelude::{Context, FullEvent};
use poise::FrameworkContext;

use crate::{Data, Error};

pub(crate) async fn handle(
    ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::CacheReady { guilds, .. } => {
            cache_ready::cache_handle(guilds, ctx, data).await;
        }
        FullEvent::ChannelCreate { channel, .. } => {
            channel::channel_create::handle_create(channel, data).await;
        }
        FullEvent::ChannelDelete { channel, .. } => {
            channel::channel_delete::handle_delete(channel, data).await;
        }
        FullEvent::ChannelUpdate { old, new, .. } => {
            channel::channel_update::handle_update(old, new, data).await;
        }
        FullEvent::GuildCreate { guild, is_new } => {
            guild::guild_create::handle_create(guild, is_new.is_some(), ctx, data).await;
        }
        FullEvent::GuildDelete { incomplete, full } => {
            guild::guild_delete::handle_delete(incomplete, full, ctx, data).await;
        }
        FullEvent::GuildUpdate {
            old_data_if_available,
            new_data,
        } => {
            guild::guild_update::handle_update(old_data_if_available, new_data, ctx, data).await;
        }
        FullEvent::GuildMemberAddition { new_member, .. } => {
            guild::guild_member_addition::handle_member_addition(new_member, ctx, data).await;
        }
        FullEvent::Ready { data_about_bot, .. } => {
            ready::handle(data_about_bot, ctx);
        }
        FullEvent::Message { new_message, .. } => {
            message::handle(new_message, ctx, data).await;
        }
        FullEvent::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id,
        } => {
            message::message_delete::handle_delete(
                channel_id,
                deleted_message_id,
                guild_id,
                ctx,
                data,
            )
            .await;
        }
        FullEvent::InteractionCreate { interaction, .. } => {
            interaction::interaction_create::handle_create(interaction, ctx, data).await;
        }
        _ => {}
    }

    Ok(())
}
