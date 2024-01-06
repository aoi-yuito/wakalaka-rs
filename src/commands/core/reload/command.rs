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

use chrono::Utc;
use serenity::all::{ ResolvedOption, CommandInteraction, ResolvedValue };
use tracing::{ log::error, log::info };

use crate::Context;

pub(super) async fn command(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>]
) -> Option<String> {
    let start_time = Utc::now();

    let existing_command_name = options
        .get(0)
        .and_then(|option| {
            match &option.value {
                ResolvedValue::String(s) => Some(*s),
                _ => None,
            }
        })
        .expect("Error while reading command name");

    let guild_id = interaction.guild_id.expect("Error while reading guild ID");

    let (existing_guild_commands, existing_global_commands) = (
        ctx.http.get_guild_commands(guild_id).await.unwrap_or_else(|why| {
            error!("Error while retrieving existing guild command(s): {why}");
            panic!("{why:?}");
        }),
        ctx.http.get_global_commands().await.unwrap_or_else(|why| {
            error!("Error while retrieving existing global command(s): {why}");
            panic!("{why:?}");
        }),
    );

    let mut existing_commands = existing_guild_commands
        .iter()
        .chain(existing_global_commands.iter());

    let commands = existing_commands.find(|command| command.name == existing_command_name);
    if let Some(command) = commands {
        let command_guild_id = command.guild_id;
        if command_guild_id.is_some() {
            let _ = ctx.http.edit_guild_command(guild_id, command.id, command);
        } else {
            let _ = ctx.http.edit_global_command(command.id, command);
        }
    }

    let elapsed_time = Utc::now() - start_time;
    let elapsed_time_millis = elapsed_time.num_milliseconds();
    let elapsed_time_secs = (elapsed_time_millis as f64) / 1000.0;
    info!("Reloaded {existing_command_name:?} in {elapsed_time_millis} milliseconds");

    Some(format!("Reloaded `{existing_command_name}` in {elapsed_time_secs:.1} seconds."))
}
