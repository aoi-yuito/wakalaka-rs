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

mod command;
mod command_panic;
mod cooldown_hit;
mod dm_only;
mod guild_only;
mod missing_bot_permissions;
mod missing_user_permissions;
mod not_an_owner;
mod nsfw_only;
mod subcommand_required;
mod unknown_command;

use crate::FrameworkError;

pub async fn handle(error: FrameworkError<'_>) {
    match error {
        FrameworkError::Setup { .. } | FrameworkError::EventHandler { .. } => {}
        FrameworkError::Command { ctx, .. } => {
            command::handle(ctx).await;
        }
        FrameworkError::SubcommandRequired { ctx, .. } => {
            subcommand_required::handle(ctx).await;
        }
        FrameworkError::CommandPanic { ctx, .. } => {
            command_panic::handle(ctx).await;
        }
        FrameworkError::ArgumentParse { .. }
        | poise::FrameworkError::CommandStructureMismatch { .. } => {}
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => cooldown_hit::handle(remaining_cooldown, ctx).await,
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            missing_bot_permissions::handle(missing_permissions, ctx).await;
        }
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            missing_user_permissions::handle(missing_permissions, ctx).await;
        }
        FrameworkError::NotAnOwner { ctx, .. } => {
            not_an_owner::handle(ctx).await;
        }
        FrameworkError::GuildOnly { ctx, .. } => {
            guild_only::handle(ctx).await;
        }
        FrameworkError::DmOnly { ctx, .. } => {
            dm_only::handle(ctx).await;
        }
        FrameworkError::NsfwOnly { ctx, .. } => {
            nsfw_only::handle(ctx).await;
        }
        FrameworkError::CommandCheckFailed { .. } | FrameworkError::DynamicPrefix { .. } => {}
        FrameworkError::UnknownCommand {
            ctx,
            msg,
            msg_content,
            ..
        } => {
            unknown_command::handle(&ctx, msg, format!("{msg_content}")).await;
        }
        FrameworkError::UnknownInteraction { .. } => {}
        FrameworkError::__NonExhaustive(_) => unreachable!(),
    }
}
