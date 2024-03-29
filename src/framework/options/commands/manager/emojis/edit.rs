// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Emoji;
use tracing::{error, info};

use crate::{
    utils::{builders, models},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_GUILD_EXPRESSIONS",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_GUILD_EXPRESSIONS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Modify an existing emoji.
pub(super) async fn edit(
    ctx: Context<'_>,
    #[description = "Emoji to rename."] emoji: Emoji,
    #[description = "New name for an emoji."]
    #[min_length = 2]
    #[max_length = 32]
    name: String,
) -> Throwable<()> {
    let guild = models::guilds::guild(ctx)?;
    let guild_name = &guild.name;

    let emoji_name = &emoji.name;
    let emoji_id = match models::emojis::emoji_id(ctx, emoji_name).await {
        Some(emoji_id) => emoji_id,
        None => {
            error!("Failed to find {emoji:?} in {guild_name}");

            let reply = builders::replies::error_reply_embed(
                format!("Cannot find `{emoji}` in {guild_name}."),
                true,
            );

            ctx.send(reply).await?;

            return Ok(());
        }
    };

    let result = match guild.edit_emoji(ctx, emoji_id, &name).await {
        Ok(_) => {
            let author = ctx.author();
            let author_name = &author.name;

            info!("@{author_name} renamed {emoji_name:?} to {name:?} in {guild_name}");
            Ok(format!("`{emoji_name}` has been renamed to `{name}`."))
        }
        Err(why) => {
            error!("Failed to rename {emoji_name:?} to {name:?} in {guild_name}: {why:?}");
            Err(format!("An error occurred while editing `{emoji}`."))
        }
    };

    let reply = match result {
        Ok(message) => builders::replies::ok_reply_embed(message, true),
        Err(message) => builders::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
