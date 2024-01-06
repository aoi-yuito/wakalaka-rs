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

use crate::{commands, Context};
use serenity::all::{CommandInteraction, CommandOptionType, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::ResolvedOption;
use tokio::time::Duration;
use tracing::log::info;

pub async fn run(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>],
) -> Option<String> {
    let administrator = crate::commands::is_administrator(ctx, interaction).await;
    if !administrator {
        return Some(format!(
            "You don't have permission(s) to execute this command!"
        ));
    }

    let command = commands::command(interaction, 0);
    match command.name.as_str() {
        "reason" => reason(ctx, options),
        "delay" => delay(options),
        _ => None,
    }
}

fn reason(ctx: &Context, options: &[ResolvedOption<'_>]) -> Option<String> {
    let reason = options
        .get(0)
        .and_then(|opt| match &opt.value {
            ResolvedValue::String(s) => Some(s),
            _ => None,
        })
        .unwrap_or(&"Cannot restart if no reason is provided.");
    if reason.len() > 50 {
        return None;
    }

    let seconds = delay(options)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(5);
    info!("Restarting in {seconds} seconds: {reason}");

    let cloned_ctx = ctx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(seconds as u64)).await;

        cloned_ctx.shard.shutdown_clean();
    });

    Some(format!("Restarting in {seconds} seconds..."))
}

fn delay(options: &[ResolvedOption<'_>]) -> Option<String> {
    let seconds = options
        .get(1)
        .and_then(|opt| match &opt.value {
            ResolvedValue::Integer(i) => Some(*i),
            _ => None,
        })
        .unwrap_or(5);
    if seconds < 5 {
        return Some("Delay cannot be less than 5 seconds.".to_string());
    } else if seconds > 60 {
        return Some("Delay cannot be more than 60 seconds (1 minute).".to_string());
    }

    None
}

pub fn register() -> CreateCommand {
    CreateCommand::new("restart")
        .description("Restarts yours truly.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "reason",
                "Short explanation for restarting.",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "delay",
                "Seconds to wait before restarting.",
            )
            .required(false),
        )
}
