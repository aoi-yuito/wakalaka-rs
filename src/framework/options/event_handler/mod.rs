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
mod message;
mod ready;
mod guild_create;

use poise::serenity_prelude::FullEvent;
use poise::FrameworkContext;

use crate::serenity::Context;
use crate::{Data, Error};

pub(crate) async fn handle(
    ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::GuildCreate { guild, is_new } => {
            guild_create::handle(guild, is_new.is_some(), ctx, data).await;
        }
        FullEvent::CacheReady { guilds, .. } => {
            cache_ready::handle(guilds, ctx).await;
        }
        FullEvent::Ready { data_about_bot, .. } => {
            ready::handle(data_about_bot, ctx);
        }
        FullEvent::Message { new_message, .. } => {
            message::handle(new_message, ctx).await;
        }
        _ => {}
    }

    Ok(())
}
