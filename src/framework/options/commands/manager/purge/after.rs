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

use serenity::{all::Message, builder::GetMessages};
use tracing::{error, info};

use crate::{
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Delete a given amount of messages after a specific message.
pub async fn after(
    ctx: Context<'_>,
    #[description = "The message to start deleting after."] message: Message,
    #[description = "The amount of messages to delete."]
    #[min = 1]
    #[max = 100]
    count: Option<u8>,
) -> Result<(), Error> {
    let count = count.unwrap_or(50);

    let http = ctx.serenity_context().http.clone();

    let user_name = ctx.author().name.clone();

    let (channel_id, channel_name) = (ctx.channel_id(), models::channels::channel_name(ctx).await?);

    let handle = tokio::spawn(async move {
        let mut deleted_message_count = 0;

        let message_id = message.id;

        let messages_builder = GetMessages::default().after(message_id).limit(count);

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

        info!("@{user_name} deleted {deleted_message_count} message(s) in #{channel_name}");

        deleted_message_count
    });

    let reply_before = messages::reply(None, "Deleting message(s)...", true);

    let reply_handle = ctx.send(reply_before).await?;

    let message_count = handle.await.unwrap_or(0);

    let reply_after = messages::ok_reply(None, format!("Deleted {message_count} message(s)."), true);
    reply_handle.edit(ctx, reply_after).await?;

    Ok(())
}
