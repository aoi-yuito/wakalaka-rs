/**
 * Copyright (C) 2024 Kawaxte
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

pub async fn on_reaction_add(
    ctx: &Context,
    reaction: Reaction,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let http = ctx.http.clone();

    Ok(())
}

pub async fn add_reaction_to_message(
    http: Arc<Http>,
    channel_id: ChannelId,
    message_id: MessageId,
    reaction: ReactionType,
) {
    let message = http
        .get_message(channel_id, message_id)
        .await
        .expect("Failed to get message");
    message
        .react(&http, reaction)
        .await
        .expect("Failed to react to message");
}
