// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::time::Duration;

use serenity::all::{Mentionable, Permissions};
use tracing::error;
use wakalaka_core::types::{Context, Error, FrameworkError};
use wakalaka_utils::builders;

pub(super) async fn handle_on_error_option(fw_error: FrameworkError<'_>) {
    match fw_error {
        FrameworkError::ArgumentParse {
            error, input, ctx, ..
        } => {
            handle_argument_parse_error(error, input, ctx).await;
        }
        FrameworkError::Command { error, ctx, .. } => {
            handle_command_error(error, ctx).await;
        }
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => {
            handle_cooldown_hit_error(remaining_cooldown, ctx).await;
        }
        FrameworkError::DmOnly { ctx, .. } => {
            handle_dm_only_error(ctx).await;
        }
        FrameworkError::GuildOnly { ctx, .. } => {
            handle_guild_only_error(ctx).await;
        }
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            handle_missing_bot_permissions_error(missing_permissions, ctx).await;
        }
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            handle_missing_user_permissions_error(missing_permissions, ctx).await;
        }
        FrameworkError::NotAnOwner { ctx, .. } => {
            handle_not_an_owner_error(ctx).await;
        }
        FrameworkError::NsfwOnly { ctx, .. } => {
            handle_nsfw_only_error(ctx).await;
        }
        FrameworkError::SubcommandRequired { ctx } => {
            handle_subcommand_required_error(ctx).await;
        }
        _ => {}
    }
}

async fn handle_argument_parse_error(e: Error, input: Option<String>, ctx: Context<'_>) {
    let command = ctx.command();
    let command_qname = &command.qualified_name;

    let result = if let Some(input) = input {
        let prefix = ctx.prefix();
        if input == prefix {
            return;
        }

        Ok(format!(
            "{input:?} is not a valid argument for `{command_qname}`."
        ))
    } else {
        error!("Failed to parse argument for /{command_qname}: {e:?}");

        Err(format!(
            "An error occurred while parsing argument for `{command_qname}`."
        ))
    };

    let reply = match result {
        Ok(text) => builders::replies::build_warning_reply_with_embed(text, true),
        Err(text) => builders::replies::build_error_reply_with_embed(text, true),
    };

    if let Err(e) = ctx.send(reply).await {
        error!("Failed to send reply: {e:?}");
    }
}

async fn handle_command_error(e: Error, ctx: Context<'_>) {
    let command = ctx.command();
    let command_qname = &command.qualified_name;

    error!("An error occurred while running /{command_qname}: {e:?}");

    let reply = builders::replies::build_error_reply_with_embed(
        format!("An error occurred while running `{command_qname}`."),
        true,
    );

    if let Err(e) = ctx.send(reply).await {
        error!("Failed to send reply: {e:?}");
    }
}

async fn handle_cooldown_hit_error(cooldown: Duration, ctx: Context<'_>) {
    let remaining_secs = cooldown.as_secs();

    let command = ctx.command();
    let command_qname = &command.qualified_name;

    let reply = if remaining_secs == 0 {
        builders::replies::build_warning_reply_with_embed(
            format!("Wait a moment before using `{command_qname}`."),
            true,
        )
    } else {
        builders::replies::build_warning_reply_with_embed(
            format!("Wait {remaining_secs} seconds before using `{command_qname}`."),
            true,
        )
    };

    if let Err(e) = ctx.send(reply).await {
        error!("Failed to send reply: {e:?}");
    }
}

async fn handle_dm_only_error(ctx: Context<'_>) {
    let command = ctx.command();
    let command_qname = &command.qualified_name;

    let reply = builders::replies::build_error_reply_with_embed(
        format!("Cannot invoke `{command_qname}` outside DMs."),
        true,
    );

    if let Err(e) = ctx.send(reply).await {
        error!("Failed to send reply: {e:?}");
    }
}

async fn handle_guild_only_error(ctx: Context<'_>) {
    let command = ctx.command();
    let command_qname = &command.qualified_name;

    let reply = builders::replies::build_error_reply_with_embed(
        format!("Cannot invoke `{command_qname}` outside a server."),
        true,
    );

    if let Err(e) = ctx.send(reply).await {
        error!("Failed to send reply: {e:?}");
    }
}

async fn handle_missing_bot_permissions_error(permissions: Permissions, ctx: Context<'_>) {
    let separated_permissions = permissions
        .iter()
        .map(|perms| format!("{perms}"))
        .collect::<Vec<_>>()
        .join(", ");

    let bot_id = ctx.framework().bot_id;
    let bot = bot_id
        .to_user(ctx)
        .await
        .expect("Failed to find user by its ID.");
    let bot_mention = bot.mention();

    let reply = builders::replies::build_error_reply_with_embed(
        format!("{bot_mention} is missing the following permissions: `{separated_permissions}`"),
        true,
    );

    if let Err(e) = ctx.send(reply).await {
        error!("Failed to send reply: {e:?}");
    }
}

async fn handle_missing_user_permissions_error(permissions: Option<Permissions>, ctx: Context<'_>) {
    let separated_permissions = permissions
        .iter()
        .map(|perms| format!("{perms}"))
        .collect::<Vec<_>>()
        .join(", ");

    let user = ctx.author();
    let user_mentionable = user.mention();

    let reply = builders::replies::build_error_reply_with_embed(
        format!(
            "{user_mentionable} is missing the following permissions: `{separated_permissions}`"
        ),
        true,
    );

    if let Err(e) = ctx.send(reply).await {
        error!("Failed to send reply: {e:?}");
    }
}

async fn handle_not_an_owner_error(ctx: Context<'_>) {
    let command = ctx.command();
    let command_qname = &command.qualified_name;

    let reply = builders::replies::build_error_reply_with_embed(
        format!("Cannot invoke `{command_qname}` without ownership."),
        true,
    );

    if let Err(e) = ctx.send(reply).await {
        error!("Failed to send reply: {e:?}");
    }
}

async fn handle_nsfw_only_error(ctx: Context<'_>) {
    let command = ctx.command();
    let command_qname = &command.qualified_name;

    let reply = builders::replies::build_error_reply_with_embed(
        format!("Cannot invoke `{command_qname}` in a channel marked as SFW."),
        true,
    );

    if let Err(e) = ctx.send(reply).await {
        error!("Failed to send reply: {e:?}");
    }
}

async fn handle_subcommand_required_error(ctx: Context<'_>) {
    let command = ctx.command();
    let command_qname = &command.qualified_name;

    let reply = builders::replies::build_error_reply_with_embed(
        format!("Cannot invoke `{command_qname}` without providing a sub-command."),
        true,
    );

    if let Err(e) = ctx.send(reply).await {
        error!("Failed to send reply: {e:?}");
    }
}
