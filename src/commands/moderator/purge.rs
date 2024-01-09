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
use tracing::info;

use crate::{Context, Error};

///Deletes messages from specified channel.
#[poise::command(slash_command, required_permissions = "MANAGE_MESSAGES")]
pub(crate) async fn purge(ctx: Context<'_>,
    #[description = "Amount of messages to delete."]
    count: u8,
) -> Result<(), Error> {
    if count < 1 || count > 100 {
        let message = format!("Sorry, but you can only delete between  and 100 messages at a time.");
        let _ = ctx.reply(message).await;

        return Ok(());
    }

    let user_name = &ctx.author().name;

    let channel_id = ctx.channel_id();
    let channel_name = channel_id.name(&ctx.http()).await?;

    let messages = GetMessages::default().limit(count);
    let channel_messages = channel_id.messages(&ctx.http(), messages).await?;

    let mut deleted_messages_count = 0;

    for channel_message in channel_messages {
        channel_message.delete(&ctx.http()).await?;

        deleted_messages_count += 1;
    }

    let reply = CreateReply {
        content: Some(format!("I've deleted {deleted_messages_count} message(s) from <#{channel_id}>")),
        ephemeral: Some(true),
        ..Default::default()
    };
    let _ = ctx.send(reply).await;

    info!("@{user_name} deleted {deleted_messages_count} message(s) from #{channel_name}");

    Ok(())
}