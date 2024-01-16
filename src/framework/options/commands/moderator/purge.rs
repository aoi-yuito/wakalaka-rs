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

use poise::CreateReply;
use serenity::builder::GetMessages;
use tracing::{error, info};

use crate::{Context, Error};

/// Deletes message(s) from provided channel.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    guild_only
)]
pub(crate) async fn purge(
    ctx: Context<'_>,
    #[description = "Amount of messages to delete."] count: u8,
) -> Result<(), Error> {
    if count < 1 || count > 100 {
        let message =
            format!("Sorry, but you can only delete between 1 and 100 messages at a time.");
        let _ = ctx.reply(message).await;

        return Ok(());
    }

    let http = ctx.serenity_context().http.clone(); // Why?
    let channel_id = ctx.channel_id();
    let user_name = ctx.author().name.clone();

    let handle = tokio::spawn(async move {
        let mut number_of_deleted_messages = 0;

        let channel_name = match channel_id.name(&http).await {
            Ok(channel_name) => channel_name,
            Err(why) => {
                error!("Couldn't get channel name: {why:?}");
                return number_of_deleted_messages;
            }
        };

        let messages = GetMessages::default().limit(count);
        let channel_messages = match channel_id.messages(&http, messages).await {
            Ok(channel_messages) => channel_messages,
            Err(why) => {
                error!("Couldn't get channel messages: {why:?}");
                return number_of_deleted_messages;
            }
        };
        for channel_message in channel_messages {
            match channel_message.delete(&http).await {
                Ok(_) => {}
                Err(why) => {
                    error!("Couldn't delete message: {why:?}");

                    // Not quite sure if returning here is a smart idea, but fuck it, unlikely anybody ever makes it here.
                    return number_of_deleted_messages;
                }
            }

            number_of_deleted_messages += 1;
        }

        info!("@{user_name} deleted {number_of_deleted_messages} message(s) from #{channel_name}");

        number_of_deleted_messages
    });

    let reply_before = CreateReply::default()
        .content("Deleting message(s)...")
        .ephemeral(true);
    let message = ctx.send(reply_before).await?;

    let number_of_deleted_messages = handle.await.unwrap_or(0);

    let reply_after = CreateReply::default()
        .content(format!(
            "I've deleted {number_of_deleted_messages} message(s) for you.",
        ))
        .ephemeral(true);
    message.edit(ctx, reply_after).await?;

    Ok(())
}
