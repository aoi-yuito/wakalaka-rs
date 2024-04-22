// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::Arc;

use serenity::all::{GetMessages, Mentionable, Message};
use tracing::{error, info};
use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::{accessors, builders};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_MESSAGES",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Delete a number of messages after specified.
pub(super) async fn after(
    ctx: Context<'_>,
    #[description = "Message to delete after."] message: Message,
    #[description = "Number of messages to delete, if any."]
    #[min = 2]
    #[max = 100]
    count: Option<u8>,
) -> Throwable<()> {
    let author = ctx.author();
    let author_name = &author.name;

    let count = count.unwrap_or(2); // "*.delete_messages()" minimum is 2.

    let raw_ctx = ctx.serenity_context();
    let raw_http = Arc::clone(&raw_ctx.http);

    let channel_id = ctx.channel_id();
    let channel_name = channel_id.name(ctx).await?;
    let channel_mention = channel_id.mention();

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = guild.name;

    let msg_id = message.id;

    let deletable_msgs = GetMessages::default().after(msg_id).limit(count);
    let deletable_msg_ids = channel_id
        .messages(&raw_http, deletable_msgs)
        .await?
        .iter()
        .map(|msg| msg.id)
        .collect::<Vec<_>>();

    let deletable_msg_count = deletable_msg_ids.len();

    let before_reply = if deletable_msg_count == 1 {
        builders::replies::build_reply_with_embed(
            format!("Deleting `{deletable_msg_count}` message..."),
            true,
        )
    } else {
        builders::replies::build_reply_with_embed(
            format!("Deleting `{deletable_msg_count}` messages..."),
            true,
        )
    };

    let result = match channel_id
        .delete_messages(&raw_http, deletable_msg_ids)
        .await
    {
        Ok(_) => {
            let success_result = match deletable_msg_count == 1 {
                true => {
                    info!("@{author_name} deleted {deletable_msg_count} message in #{channel_name} in {guild_name}");

                    Ok(format!("`{deletable_msg_count}` message has been deleted."))
                }
                false => {
                    info!("@{author_name} deleted {deletable_msg_count} messages in #{channel_name} in {guild_name}");

                    Ok(format!(
                        "`{deletable_msg_count}` messages have been deleted."
                    ))
                }
            };
            success_result
        }
        Err(e) => {
            let error_result = match deletable_msg_count == 1 {
                true => {
                    error!("@{author_name} failed to delete {deletable_msg_count} message in #{channel_name} in {guild_name}: {e:?}");

                    Err(format!("An error occurred while deleting {deletable_msg_count} in {channel_mention}."))
                }
                false => {
                    error!("@{author_name} failed to delete {deletable_msg_count} messages in #{channel_name} in {guild_name}: {e:?}");

                    Err(format!("An error occurred while deleting {deletable_msg_count} in {channel_mention}."))
                }
            };
            error_result
        }
    };

    let reply_handle = ctx.send(before_reply).await?;

    let after_reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(e) => builders::replies::build_error_reply_with_embed(e, true),
    };

    reply_handle.edit(ctx, after_reply).await?;

    Ok(())
}
