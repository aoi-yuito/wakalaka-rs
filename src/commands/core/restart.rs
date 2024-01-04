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

use std::time::Duration;

use serenity::all::{CommandInteraction, CommandOptionType, ResolvedValue};

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::ResolvedOption;

use crate::Context;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>],
) -> String {
    let administrator_permission =
        crate::commands::has_administrator_permission(ctx, interaction).await;
    if !administrator_permission {
        return "You don't have rights to execute this command!".to_string();
    }

    let timer = options
        .get(0)
        .and_then(|opt| match &opt.value {
            ResolvedValue::Integer(i) => Some(*i),
            _ => None,
        })
        .unwrap_or(5);
    if timer < 5 {
        return "Cannot restart in less than 5 seconds.".to_string();
    } else if timer > 60 {
        return "Cannot restart in more than 1 minute.".to_string();
    }

    let cloned_ctx = ctx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(timer as u64)).await;

        cloned_ctx.shard.shutdown_clean();
    });

    return format!("Restarting in {timer} seconds...",);
}

pub fn register() -> CreateCommand {
    CreateCommand::new("restart")
        .description("Restarts the bot.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "seconds",
                "Delay in seconds before restarting.",
            )
            .required(false),
        )
}
