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

pub mod message_delete;

use serenity::{all::Message, futures::StreamExt};
use tokio::time::{Duration, Instant};

use crate::{utility::components::messages, Data, Error};

pub async fn handle(
    msg: &Message,
    ctx: &crate::serenity::Context,
    data: &Data,
) -> Result<(), Error> {
    let _pool = &data.pool;

    let member = msg.member(&ctx).await?;
    if member.permissions(&ctx)?.administrator() {
        return Ok(());
    }

    let user_id = msg.author.id;

    let start_time = Instant::now();

    let amount_of_messages = data.amount_of_messages.lock().await;

    let mut message_count = amount_of_messages.entry(user_id).or_insert((0, start_time));
    message_count.0 += 1;

    let elapsed_time = message_count.1.elapsed();

    if message_count.0 > 8 && elapsed_time < Duration::from_secs(5) {
        limit_messages_per_user(msg, ctx).await?;
    }
    if elapsed_time >= Duration::from_secs(5) {
        *message_count = (0, Instant::now());
    }

    remove_invite_links(msg, ctx).await?;

    Ok(())
}

async fn limit_messages_per_user(
    msg: &Message,
    ctx: &crate::serenity::Context,
) -> Result<(), Error> {
    let user = &msg.author;
    if user.bot || user.system {
        return Ok(());
    }

    let channel_id = msg.channel_id;

    let messages = channel_id.messages_iter(&ctx).filter_map(|message| async {
        let message = message.ok()?;

        if message.author.id == user.id {
            Some(message)
        } else {
            None
        }
    });
    for message in messages.take(8).collect::<Vec<Message>>().await {
        message.delete(&ctx).await?;
    }

    let message = messages::warn_message("Quiet down! You're sending messages too quickly!");
    user.direct_message(&ctx, message).await?;

    Ok(())
}

async fn remove_invite_links(msg: &Message, ctx: &crate::serenity::Context) -> Result<(), Error> {
    let user = &msg.author;
    if user.bot || user.system {
        return Ok(());
    }

    let message_content = msg.content_safe(&ctx);

    if message_content.contains("discord.gg/")
        || message_content.contains("discord.com/invite/")
        // Legacy failsafe to prevent people from bypassing the filter
        || message_content.contains("discordapp.com/invite/")
    {
        msg.delete(&ctx).await?;

        let message = messages::warn_message("Hey! You're not allowed to advertise here!");
        user.direct_message(&ctx, message).await?;
    }

    Ok(())
}
