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

pub async fn message(ctx: &Context, msg: &Message) -> Result<(), crate::Error> {
    let http = ctx.http.clone();

    read_image_attachment_metadata(http, msg).await;

    Ok(())
}

async fn read_image_attachment_metadata(http: Arc<Http>, msg: &Message) {
    let attachments = &msg.attachments;
    if !attachments.is_empty() {
        let mut metadata = HashMap::new();

        for (index, attachment) in attachments.iter().enumerate() {
            FileMetadata::read(index, attachment, &mut metadata)
                .await
                .expect("Failed to read metadata");
        }

        let channel_id = msg.channel_id;
        let message_id = msg.id;

        add_reaction_to_message(
            http,
            channel_id,
            message_id,
            ReactionType::Unicode("🔎".to_string()),
        )
        .await;
    }
}

async fn add_reaction_to_message(
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
