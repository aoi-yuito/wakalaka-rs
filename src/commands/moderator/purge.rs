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

/// Deletes message(s) from specified channel.
#[poise::command(slash_command, required_permissions = "MANAGE_MESSAGES")]
pub(crate) async fn purge(ctx: Context<'_>,
    #[description = "Amount of messages to delete."]
    count: u8,
) -> Result<(), Error> {
    if count < 1 || count > 100 {
        let message = format!("Sorry, but you can only delete between 1 and 100 messages at a time.");
        let _ = ctx.reply(message).await;

        return Ok(());
    }

    let http = ctx.serenity_context().http.clone();

    let user_name = ctx.author().name.clone();

    let channel_id = ctx.channel_id();
    let channel_name = channel_id.name(&ctx.http()).await?;

    let mut number_of_deleted_messages = 0;

    let messages = GetMessages::default().limit(count);
    let channel_messages = channel_id.messages(&http, messages).await?;
    for channel_message in channel_messages {
        channel_message.delete(&http).await?;

        number_of_deleted_messages += 1;
    }

    info!("@{user_name} deleted {number_of_deleted_messages} message(s) from #{channel_name}");

    let reply = CreateReply {
        content: Some(format!("Deleted {number_of_deleted_messages} message(s).")),
        ephemeral: Some(true),
        ..Default::default()
    };
    let _ = ctx.send(reply).await;

    Ok(())
}