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

pub fn is_message_embed(msg: &Message) -> bool {
    !msg.embeds.is_empty()
}

pub async fn on_message(ctx: &Context, msg: &Message) -> Result<(), crate::Error> {
    let http = ctx.http.clone();

    FileMetadata::attachment_metadata(http, msg).await;

    Ok(())
}

pub async fn send_dm(
    http: Arc<Http>,
    user_id: UserId,
    s: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let dm_channel = user_id.create_dm_channel(&http).await?;
    dm_channel.say(&http, s).await?;

    Ok(())
}

pub async fn send_dm_as_embed(
    http: Arc<Http>,
    user_id: UserId,
    message: CreateMessage,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let dm_channel = user_id.create_dm_channel(&http).await?;
    dm_channel.send_message(&http, message).await?;

    Ok(())
}
