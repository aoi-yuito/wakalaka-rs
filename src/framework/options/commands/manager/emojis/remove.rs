// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::{error, info};

use crate::{
    utils::{builders, models},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "CREATE_GUILD_EXPRESSIONS",
    required_bot_permissions = "SEND_MESSAGES | CREATE_GUILD_EXPRESSIONS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Delete an existing emoji.
pub(super) async fn remove(
    ctx: Context<'_>,
    #[description = "The name of the emoji."]
    #[min_length = 2]
    #[max_length = 32]
    name: String,
) -> Throwable<()> {
    let guild = models::guilds::guild(ctx)?;
    let guild_name = &guild.name;

    let emoji_id = match models::emojis::emoji_id(ctx, &name).await {
        Some(emoji_id) => emoji_id,
        None => {
            error!("Failed to find {name:?} in {guild_name}");

            let reply =
                builders::replies::error_reply_embed(format!("`{name}` does not exist!"), true);

            ctx.send(reply).await?;

            return Ok(());
        }
    };
    let emoji = models::emojis::emoji(ctx, emoji_id).await?;
    let emoji_name = &emoji.name;

    let result = match guild.delete_emoji(ctx, emoji_id).await {
        Ok(_) => {
            let author = ctx.author();
            let author_name = &author.name;

            info!("@{author_name} deleted {emoji_name:?} from {guild_name}");
            Ok(format!("`{emoji_name}` has been deleted."))
        }
        Err(why) => {
            error!("Failed to delete {emoji_name:?} from {guild_name}: {why:?}");
            Err(format!("An error occurred while deleting `{emoji_name}`."))
        }
    };

    let reply = match result {
        Ok(message) => builders::replies::ok_reply_embed(message, true),
        Err(message) => builders::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
