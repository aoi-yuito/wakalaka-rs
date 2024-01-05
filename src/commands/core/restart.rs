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
use tracing::log::info;

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

    let seconds = match seconds(options) {
        Ok(value) => value,
        Err(value) => return value,
    };
    let reason = match reason(options) {
        Ok(value) => value,
        Err(value) => return value,
    };

    let cloned_ctx = ctx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(seconds as u64)).await;

        cloned_ctx.shard.shutdown_clean();
    });

    info!("Restarting in {seconds} seconds: {reason}");
    return "Restarting...".to_string();
}

fn reason(options: &[ResolvedOption<'_>]) -> Result<String, String> {
    let reason = options
        .get(0)
        .and_then(|opt| match &opt.value {
            ResolvedValue::String(s) => Some(s),
            _ => None,
        })
        .unwrap_or(&"Cannot restart if no reason is provided.");
    if reason.len() > 50 {
        return Err("Reason cannot be longer than 50 characters.".to_string());
    }
    Ok(reason.to_string())
}

fn seconds(options: &[ResolvedOption<'_>]) -> Result<i64, String> {
    let seconds = options
        .get(1)
        .and_then(|opt| match &opt.value {
            ResolvedValue::Integer(i) => Some(*i),
            _ => None,
        })
        .unwrap_or(5);
    if seconds < 5 {
        return Err("Delay cannot be less than 5 seconds.".to_string());
    } else if seconds > 60 {
        return Err("Delay cannot be more than 60 seconds (1 minute).".to_string());
    }
    Ok(seconds)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("restart")
        .description("Restarts the bot.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "reason",
                "Reason for restarting the bot.",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "seconds",
                "Delay in seconds before restarting.",
            )
            .required(false),
        )
}
