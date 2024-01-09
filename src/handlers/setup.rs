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

use tracing::{error, info, warn};

use crate::serenity::Context;
use crate::{commands, util, Data, Error};

pub(crate) async fn handle(ctx: &Context) -> Result<Data, Error> {
    register_guild_commands(ctx).await;

    Ok(Data {
        suggestion_id: AtomicUsize::new(1),
        restricted_channels: Default::default(),
    })
}

// async fn unregister_global_commands(ctx: &Context) {
//     let global_commands = ctx
//         .http
//         .get_global_commands().await
//         .unwrap();
//     let global_command_count = global_commands.len();

//     for global_command in global_commands {
//         ctx.http.delete_global_command(global_command.id).await.unwrap();
//     }

//     info!("Unregistered {global_command_count} global command(s)");
// }

// async fn unregister_guild_commands(ctx: &Context) {
//     let guild_id = match util::guild_id_raw(ctx).await {
//         Some(value) => value,
//         None => return,
//     };

//     let guild_commands = ctx
//         .http
//         .get_guild_commands(guild_id).await
//         .unwrap();
//     let guild_command_count = guild_commands.len();

//     for guild_command in guild_commands {
//         ctx.http.delete_guild_command(guild_id, guild_command.id).await.unwrap();
//     }

//     info!("Unregistered {guild_command_count} guild command(s)");
// }

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
    //if none
    if guild_command_count == 0 {
        warn!("No guild command(s) to register in {guild_name}");
        return;
    }

    match poise::builtins::register_in_guild(&ctx.http, &guild_commands, guild_id).await {
        Ok(_) => {}
        Err(why) => {
            error!("Couldn't register guild commands: {why:?}");
            panic!("{why:?}");
        }
    }
    info!("Registered {guild_command_count} command(s) in {guild_name}");
}
