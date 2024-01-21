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

use serenity::all::{Emoji, EmojiId};
use tracing::error;

use crate::{utility::models, Context};

pub(crate) async fn emoji_id(ctx: Context<'_>, name: &str) -> Option<EmojiId> {
    let emojis = emojis(ctx).await;
    for emoji in emojis {
        let (emoji_name, emoji_id) = (emoji.name, emoji.id);
        if emoji_name == name {
            return Some(emoji_id);
        }
    }
    None
}

pub(crate) async fn emoji(ctx: Context<'_>, id: EmojiId) -> Option<Emoji> {
    let guild = models::guilds::guild(ctx).await;

    match guild.emoji(&ctx, id).await {
        Ok(emoji) => Some(emoji),
        Err(why) => {
            error!("Couldn't get emoji: {why:?}");
            None
        }
    }
}

pub(crate) async fn emojis(ctx: Context<'_>) -> Vec<Emoji> {
    let guild = models::guilds::guild(ctx).await;

    match guild.emojis(&ctx).await {
        Ok(emojis) => emojis,
        Err(why) => {
            error!("Couldn't get emojis: {why:?}");
            return Vec::new();
        }
    }
}
