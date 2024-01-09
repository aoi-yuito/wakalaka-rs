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

use std::sync::atomic::AtomicUsize;

use tracing::{error, info};

use crate::serenity::Context;
use crate::{commands, util, Data, Error};

pub(crate) async fn handle(ctx: &Context) -> Result<Data, Error> {
    register_guild_commands(ctx).await;

    let data = Data {
        suggestion_id: AtomicUsize::new(1),
        restricted_channels: Default::default(),
    };
    Ok(data)
}

async fn register_guild_commands(ctx: &Context) {
    let guild_id = match util::guild_id_raw(ctx).await {
        Some(value) => value,
        None => return,
    };
    let guild_name = match util::guild_name_raw(&guild_id, ctx) {
        Some(value) => value,
        None => return,
    };

    let guild_commands = commands::guild_commands().await;
    let guild_command_count = guild_commands.len();

    match poise::builtins::register_in_guild(&ctx.http, &guild_commands, guild_id).await {
        Ok(_) => {}
        Err(why) => {
            error!("Couldn't register guild commands: {why:?}");
            panic!("{why:?}");
        }
    }
    info!("Registered {guild_command_count} command(s) in {guild_name}");
}
