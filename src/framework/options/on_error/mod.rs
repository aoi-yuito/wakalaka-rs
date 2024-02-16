// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod cooldown_hit;
mod dm_only;
mod guild_only;
mod missing_bot_permissions;
mod missing_user_permissions;
mod not_an_owner;
mod nsfw_only;

use crate::FrameworkError;

pub(crate) async fn handle(framework_error: FrameworkError<'_>) {
    match framework_error {
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => missing_bot_permissions::handle(missing_permissions, ctx).await,
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
            ..
        } => missing_user_permissions::handle(missing_permissions, ctx).await,
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => cooldown_hit::handle(remaining_cooldown, ctx).await,
        FrameworkError::NotAnOwner { ctx, .. } => not_an_owner::handle(ctx).await,
        FrameworkError::GuildOnly { ctx, .. } => guild_only::handle(ctx).await,
        FrameworkError::DmOnly { ctx, .. } => dm_only::handle(ctx).await,
        FrameworkError::NsfwOnly { ctx, .. } => nsfw_only::handle(ctx).await,
        _ => {}
    }
}
