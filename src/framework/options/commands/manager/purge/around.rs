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

use crate::{utility::components::messages, Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    guild_only,
    ephemeral
)]
/// Delete a given amount of messages around a specific message.
pub(crate) async fn around(
    ctx: Context<'_>,
    #[description = "The message to delete around."] message: Message,
    #[description = "The amount to delete around. (1-100)"]
    #[min = 1]
    #[max = 100]
    count: Option<u8>,
) -> Result<(), Error> {
    let count = count.unwrap_or(1);
    if count < 1 || count > 100 {
        let reply = messages::warn_reply("Amount must be between 1 and 100 message(s).", true);
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

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

        let message_id = message.id;

        let messages_around = GetMessages::default().around(message_id).limit(count);
        let messages = match channel_id.messages(&http, messages_around).await {
            Ok(messages) => messages,
            Err(why) => {
                error!("Couldn't get messages: {why:?}");
                return number_of_deleted_messages;
            }
        };
        for message in messages {
            if let Err(why) = message.delete(&http).await {
                error!("Couldn't delete message: {why:?}");
                continue;
            }

            number_of_deleted_messages += 1;
        }

        info!("@{user_name} deleted {number_of_deleted_messages} message(s) in #{channel_name}");

        number_of_deleted_messages
    });

    let reply_before = messages::reply("Deleting message(s)...", true);
    let reply = ctx.send(reply_before).await?;

    let number_of_deleted_messages = handle.await.unwrap_or(0);

    let reply_after = messages::ok_reply(
        format!("Deleted {number_of_deleted_messages} message(s)."),
        true,
    );
    reply.edit(ctx, reply_after).await?;

    Ok(())
}
