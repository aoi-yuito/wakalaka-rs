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

use serenity::{ all::{ CommandInteraction, ResolvedOption, ResolvedValue }, builder::GetMessages };
use tracing::{ log::error, log::info };

use crate::Context;
use crate::commands;

pub(super) async fn count(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>]
) -> Option<String> {
    let manage_messages = commands::has_manage_messages_permission(ctx, interaction).await;
    if !manage_messages {
        return Some("You don't have permission to delete messages!".to_string());
    }

    let count = options
        .get(0)
        .and_then(|option| {
            match &option.value {
                ResolvedValue::Integer(i) => Some(*i as u8),
                _ => None,
            }
        })
        .unwrap_or(1);
    if count > 100 {
        return Some("Can't delete more than 100 messages at once!".to_string());
    }

    let messages = GetMessages::default().limit(count);

    let mut deleted_message_count = 0;

    let channel_id = interaction.channel_id;
    let (channel_name, channel_messages) = (
        channel_id.name(&ctx.http).await.unwrap_or_else(|why| {
            error!("Error while retrieving channel name: {why}");
            panic!("{why:?}");
        }),
        channel_id.messages(&ctx.http, messages).await.unwrap_or_else(|why| {
            error!("Error while retrieving messages: {why}");
            panic!("{why:?}");
        }),
    );
    for channel_message in channel_messages {
        channel_message.delete(&ctx.http).await.unwrap_or_else(|why| {
            error!("Error while deleting message: {why}");
            panic!("{why:?}");
        });

        deleted_message_count += 1;
    }

    let user_name = &interaction.user.name;
    info!("{user_name} deleted {deleted_message_count} message(s) from #{channel_name}.");

    Some(format!("Deleted {deleted_message_count} message(s)!"))
}
