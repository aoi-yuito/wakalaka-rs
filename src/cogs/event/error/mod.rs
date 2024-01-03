mod argument_parse;
mod command;
mod command_structure_mismatch;
mod cooldown_hit;
mod missing_permissions;
mod only;
mod setup;

/**
 * Copyright (C) 2024 Kasutaja
 *
 * wakalaka-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * wakalaka-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::uses::*;

pub async fn on_error(error: FrameworkError<'_, crate::Data, crate::Error>) {
    match error {
        FrameworkError::Setup { error, .. } => {
            setup::on_setup(error).await;
        }
        FrameworkError::Command { error, ctx, .. } => {
            command::on_command(error, ctx).await;
        }
        FrameworkError::ArgumentParse {
            error, input, ctx, ..
        } => {
            argument_parse::on_argument_parse(error, input, ctx).await;
        }
        FrameworkError::CommandStructureMismatch {
            description, ctx, ..
        } => {
            command_structure_mismatch::on_command_structure_mismatch(description, ctx).await;
        }
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => {
            cooldown_hit::on_cooldown_hit(remaining_cooldown, ctx).await;
        }
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            missing_permissions::on_missing_bot_permissions(missing_permissions, ctx).await;
        }
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            missing_permissions::on_missing_user_permissions(missing_permissions, ctx).await;
        }
        FrameworkError::NotAnOwner { ctx, .. } => {
            only::on_not_an_owner(ctx).await;
        }
        FrameworkError::GuildOnly { ctx, .. } => {
            only::on_guild_only(ctx).await;
        }
        FrameworkError::DmOnly { ctx, .. } => {
            only::on_dm_only(ctx).await;
        }
        FrameworkError::NsfwOnly { ctx, .. } => {
            only::on_nsfw_only(ctx).await;
        }
        error => {
            if let Err(e) = builtins::on_error(error).await {
                eprintln!("Error: {e}");
            }
        }
    }
}
