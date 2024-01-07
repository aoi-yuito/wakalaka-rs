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

use crate::Context;

use std::sync::atomic::{ AtomicUsize, Ordering };
use serenity::{
    all::{
        ResolvedOption,
        ResolvedValue,
        CommandInteraction,
        colours::branding,
        ReactionType,
        ChannelId,
    },
    builder::{
        CreateEmbedAuthor,
        CreateEmbed,
        CreateMessage,
        GetMessages,
        CreateInteractionResponseMessage,
        CreateButton,
    },
};

static SUGGESTION_ID: AtomicUsize = AtomicUsize::new(1);

pub(super) async fn message(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[ResolvedOption<'_>]
) -> Option<String> {
    let description = options
        .get(0)
        .and_then(|option| {
            match &option.value {
                ResolvedValue::String(s) => Some(s),
                _ => None,
            }
        })
        .expect("Error while getting description")
        .trim();

    let description_character_count = description.chars().count();
    if description_character_count > 400 {
        return Some(format!("Description can't be longer than 400 characters!"));
    } else if description_character_count < 50 {
        return Some(format!("Description can't be shorter than 50 characters!"));
    }

    let user = &interaction.user;
    let user_name = &user.name;
    let user_avatar_url = user.avatar_url().unwrap_or(user.default_avatar_url());

    let embed = CreateEmbed::default()
        .title(format!("Suggestion #{}", SUGGESTION_ID.fetch_add(1, Ordering::Relaxed)))
        .author(CreateEmbedAuthor::new(user_name).icon_url(user_avatar_url))
        .description(description)
        .color(branding::BLURPLE);

    let guild_id = interaction.guild_id.expect("Error while getting guild ID");

    let channel_ids = guild_id.channels(&ctx.http).await.expect("Error while getting channels");
    let channel_id = channel_ids
        .values()
        .find(|channel| channel.name == "suggestions")
        .expect("Error while getting #suggestions channel").id;
    if let Some(channel) = channel_ids.get(&channel_id) {
        let message = CreateMessage::new().embed(embed);
        let suggest_message = channel
            .send_message(&ctx.http, message).await
            .expect("Error while sending suggestion message");

        let thumbs_up = ReactionType::Unicode(format!("👍"));
        let thumbs_down = ReactionType::Unicode(format!("👎"));
        for reaction in &[thumbs_up, thumbs_down] {
            suggest_message
                .react(&ctx.http, reaction.clone()).await
                .expect("Error while reacting to suggestion message");
        }
    }
    None
}
