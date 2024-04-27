// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Emoji;

use wakalaka_core::{
    accessors, builders,
    types::{Context, Throwable},
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
/// Delete an existing emoji.
pub(super) async fn delete(
    ctx: Context<'_>,
    #[description = "Emoji to delete."] emoji: Emoji,
) -> Throwable<()> {
    let author = ctx.author();
    let author_name = &author.name;

    let emoji_id = &emoji.id;
    let emoji_name = &emoji.name;

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = &guild.name;

    let result = match guild.delete_emoji(ctx, emoji_id).await {
        Ok(_) => {
            tracing::info!("@{author_name} deleted :{emoji_name}: in {guild_name}");

            Ok(format!("{emoji_name:?} has been deleted."))
        }
        Err(e) => {
            tracing::error!(
                "@{author_name} failed to delete :{emoji_name}: in {guild_name}: {e:?}"
            );

            Err(format!("An error occurred while deleting {emoji_name:?}."))
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(emsg) => builders::replies::build_error_reply_with_embed(emsg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
