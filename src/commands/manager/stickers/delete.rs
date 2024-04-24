// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::StickerId;

use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::{accessors, builders};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "CREATE_GUILD_EXPRESSIONS",
    required_bot_permissions = "SEND_MESSAGES | CREATE_GUILD_EXPRESSIONS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Delete an existing sticker.
pub(super) async fn delete(
    ctx: Context<'_>,
    #[description = "Sticker to delete."]
    #[rename = "sticker"]
    sticker_id: StickerId,
) -> Throwable<()> {
    let author = ctx.author();
    let author_name = &author.name;

    let sticker = sticker_id.to_sticker(ctx).await?;
    let sticker_name = &sticker.name;

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = &guild.name;

    let result = match guild.delete_sticker(ctx, sticker_id).await {
        Ok(_) => {
            tracing::info!("@{author_name} deleted {sticker_name:?} in {guild_name}");

            Ok(format!("`{sticker_name}` has been delete."))
        }
        Err(e) => {
            tracing::error!(
                "@{author_name} failed to delete {sticker_name:?} in {guild_name}: {e:?}"
            );

            Err(format!(
                "An error occurred while deleting `{sticker_name}`."
            ))
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(emsg) => builders::replies::build_error_reply_with_embed(emsg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
