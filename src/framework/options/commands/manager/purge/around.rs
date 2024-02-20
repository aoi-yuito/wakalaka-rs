// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::Arc;

use serenity::{all::Message, builder::GetMessages};
use tracing::error;

use crate::{utils::components, Context, Error};

#[poise::command(
    slash_command,
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    required_bot_permissions = "MANAGE_GUILD | SEND_MESSAGES | MANAGE_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Delete a given amount of messages around a specific message.
pub(super) async fn around(
    ctx: Context<'_>,
    #[description = "The message to start deleting from."] message: Message,
    #[description = "The amount of messages to delete."]
    #[min = 1]
    #[max = 100]
    count: Option<u8>,
) -> Result<(), Error> {
    let count = count.unwrap_or(50);

    let s_ctx = ctx.serenity_context();
    let http = Arc::clone(&s_ctx.http);

    let channel_id = ctx.channel_id();

    let handle = tokio::spawn(async move {
        let mut deleted_message_count = 0;

        let message_id = message.id;

        let messages_builder = GetMessages::default().around(message_id).limit(count);

        let messages = match channel_id.messages(&http, messages_builder).await {
            Ok(messages) => messages,
            Err(why) => {
                error!("Failed to get messages: {why:?}");
                return deleted_message_count;
            }
        };
        for message in messages {
            if let Err(why) = message.delete(&http).await {
                error!("Failed to delete message: {why:?}");
                continue;
            }

            deleted_message_count += 1;
        }

        deleted_message_count
    });

    let reply_before = components::replies::reply_embed(format!("Deleting messages..."), true);

    let reply_handle = ctx.send(reply_before).await?;

    let deleted_message_count = handle.await?;
    if deleted_message_count == 0 {
        let reply =
            components::replies::warn_reply_embed(format!("No messages were deleted."), true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let reply_after = if deleted_message_count == 1 {
        components::replies::ok_reply_embed(
            format!("{deleted_message_count} message has been deleted."),
            true,
        )
    } else {
        components::replies::ok_reply_embed(
            format!("{deleted_message_count} messages have been deleted."),
            true,
        )
    };

    reply_handle.edit(ctx, reply_after).await?;

    Ok(())
}
