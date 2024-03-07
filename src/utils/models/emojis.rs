// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Emoji, EmojiId};
use tracing::{error, warn};

use crate::{utils::models, Context, Error};

pub(crate) async fn emoji_id(ctx: Context<'_>, name: &String) -> Option<EmojiId> {
    match emojis(ctx).await {
        Ok(emojis) => {
            for emoji in emojis {
                let emoji_id = emoji.id;
                let emoji_name = &emoji.name;
                if emoji_name == name {
                    Some(emoji_id);
                }
            }

            warn!("No emoji found: {name}");
            None
        }
        Err(why) => {
            error!("Failed to get emoji ID: {why:?}");
            None
        }
    }
}

pub(crate) async fn emojis(ctx: Context<'_>) -> Result<Vec<Emoji>, Error> {
    let guild = models::guilds::guild(ctx)?;

    match guild.emojis(ctx).await {
        Ok(emojis) => Ok(emojis),
        Err(why) => {
            error!("Failed to get emojis: {why:?}");
            Err(why.into())
        }
    }
}

pub(crate) async fn emoji(ctx: Context<'_>, emoji_id: EmojiId) -> Result<Emoji, Error> {
    let guild = models::guilds::guild(ctx)?;

    match guild.emoji(ctx, emoji_id).await {
        Ok(emoji) => Ok(emoji),
        Err(why) => {
            error!("Failed to get emoji: {why:?}");
            Err(why.into())
        }
    }
}
