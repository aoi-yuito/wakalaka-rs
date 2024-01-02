/**
 * Copyright (C) 2024 Kasutaja
 *
 * wakalaka-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * wakalaka-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::util::uses::*;

pub fn is_embed(msg: &Message) -> bool {
    !msg.embeds.is_empty()
}

pub fn is_attachment(msg: &Message) -> bool {
    !msg.attachments.is_empty()
}

pub async fn on_message(ctx: &Context, msg: &Message) -> Result<(), crate::Error> {
    let http = ctx.http.clone();

    FileMetadata::attachment_metadata(ctx, http, msg).await;

    Ok(())
}

pub async fn send_dm_raw(
    http: Arc<Http>,
    user_id: UserId,
    s: &str,
) -> Result<(), Box<dyn error::Error + Send + Sync>> {
    let dm_channel = user_id.create_dm_channel(&http).await?;
    dm_channel.say(&http, s).await?;

    Ok(())
}

pub async fn send_dm(
    http: Arc<Http>,
    user_id: UserId,
    message: CreateMessage,
) -> Result<(), Box<dyn error::Error + Send + Sync>> {
    let dm_channel = user_id.create_dm_channel(&http).await?;
    dm_channel.send_message(&http, message).await?;

    Ok(())
}

pub async fn send_dm_if_embed_attachment_raw(
    reaction: Reaction,
    channel_id: u64,
    http: Arc<Http>,
    ctx: &Context,
    mag_right: ReactionType,
    s: &str,
) -> Result<(), Box<dyn error::Error + Send + Sync>> {
    let reaction_channel_id = reaction.channel_id;
    if reaction_channel_id != ChannelId::from(channel_id) {
        return Ok(());
    }

    let message_id = reaction.message_id;
    let reactive_message = http
        .get_message(reaction_channel_id, message_id)
        .await
        .expect("Failed to get message");

    let user_id = reaction.user_id;

    if event::message::is_attachment(&reactive_message)
        || event::message::is_embed(&reactive_message)
    {
        let users = reactive_message
            .reaction_users(&ctx, mag_right.clone(), None, None)
            .await?;
        for user in users {
            if user.bot {
                continue;
            }

            let _ = event::message::send_dm_raw(http.clone(), user_id.unwrap(), s).await;
        }
    }

    Ok(())
}

pub async fn send_dm_if_embed_attachment(
    reaction: Reaction,
    channel_id: u64,
    http: Arc<Http>,
    ctx: &Context,
    mag_right: ReactionType,
    message: CreateMessage,
) -> Result<(), Box<dyn error::Error + Send + Sync>> {
    let reaction_channel_id = reaction.channel_id;
    if reaction_channel_id != ChannelId::from(channel_id) {
        return Ok(());
    }

    let message_id = reaction.message_id;
    let reactive_message = http
        .get_message(reaction_channel_id, message_id)
        .await
        .expect("Failed to get message");

    let user_id = reaction.user_id;

    if event::message::is_attachment(&reactive_message)
        || event::message::is_embed(&reactive_message)
    {
        let users = reactive_message
            .reaction_users(&ctx, mag_right.clone(), None, None)
            .await?;
        for user in users {
            if user.bot {
                continue;
            }

            let _ = event::message::send_dm(http.clone(), user_id.unwrap(), message.clone()).await;
        }
    }

    Ok(())
}
