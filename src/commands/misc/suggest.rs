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

use std::sync::atomic::Ordering;

use serenity::{
    all::{PermissionOverwrite, PermissionOverwriteType, Permissions, ReactionType},
    builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage},
};
use tracing::error;

use crate::{Context, Error, check_channel_restriction};

/// Suggest things for yours truly, or for community.
#[poise::command(slash_command)]
pub(crate) async fn suggest(
    ctx: Context<'_>,
    #[description = "Brief overview of your suggestion."] message: String,
) -> Result<(), Error> {
    check_channel_restriction!(ctx);
    
    let suggestion_id = &ctx.data().suggestion_id;

    let message_character_count = message.chars().count();
    if message_character_count < 10 || message_character_count > 2000 {
        let message_ = format!("Suggestion must be between 10 and 2000 characters!");
        let _ = ctx.reply(message_).await;

        return Ok(());
    }

    let guild_id = match ctx.guild_id() {
        Some(value) => value,
        None => return Ok(()),
    };

    let guild_channels = match ctx.http().get_channels(guild_id).await {
        Ok(value) => value,
        Err(why) => return Err(why.into()),
    };

    let suggestions_channel = guild_channels
        .iter()
        .find(|channel| channel.name == "suggestions");
    if let Some(channel) = suggestions_channel {
        let suggestions_channel_id = channel.id;
        let bot_id = ctx.cache().current_user().id;

        let permissions = PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(bot_id),
        };
        if let Err(why) = suggestions_channel_id
            .create_permission(&ctx.http(), permissions)
            .await
        {
            error!("Couldn't create permission overwrite: {why:?}");
        }

        let user_name = &ctx.author().name;
        let user_avatar_url = match ctx.author().avatar_url() {
            Some(url) => url,
            None => ctx.author().default_avatar_url(),
        };

        let embed = embed(
            suggestion_id.fetch_add(1, Ordering::Relaxed),
            user_name,
            user_avatar_url,
            message,
        );
        let message = CreateMessage::default().embed(embed);
        let message_handle = match suggestions_channel_id
            .send_message(&ctx.http(), message)
            .await
        {
            Ok(value) => value,
            Err(why) => return Err(why.into()),
        };

        let (thumbs_up, thumbs_down) = (
            ReactionType::Unicode("👍".into()),
            ReactionType::Unicode("👎".into()),
        );
        for reaction in &[thumbs_up, thumbs_down] {
            let _ = message_handle.react(&ctx.http(), reaction.clone()).await;
        }
    }

    let _ = ctx.reply(format!("Your suggestion has been sent!")).await;

    Ok(())
}

fn embed(id: usize, name: &String, avatar_url: String, description: String) -> CreateEmbed {
    CreateEmbed::default()
        .title(format!("Suggestion #{}", id))
        .author(embed_author(name, avatar_url))
        .description(description)
}

fn embed_author(user_name: &String, user_avatar_url: String) -> CreateEmbedAuthor {
    CreateEmbedAuthor::new(user_name).icon_url(user_avatar_url)
}
