// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{EditSticker, StickerId};

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
/// Edit an existing sticker.
pub(super) async fn edit(
    ctx: Context<'_>,
    #[description = "Sticker to edit."]
    #[rename = "sticker"]
    sticker_id: StickerId,
    #[description = "Name to set."]
    #[min_length = 2]
    #[max_length = 30]
    name: String,
    #[description = "Description to set, if any."]
    #[min_length = 2]
    #[max_length = 100]
    description: Option<String>,
) -> Throwable<()> {
    let author = ctx.author();
    let author_name = &author.name;

    let sticker = sticker_id.to_sticker(ctx).await?;
    let sticker_name = &sticker.name;

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = &guild.name;

    let editable_sticker = EditSticker::new()
        .name(&name)
        .description(description.unwrap_or_default());

    let result = match guild.edit_sticker(ctx, sticker_id, editable_sticker).await {
        Ok(_) => {
            tracing::info!("@{author_name} edited {sticker_name:?} in {guild_name}");

            Ok(format!("`{sticker_name}` has been edited."))
        }
        Err(e) => {
            tracing::error!(
                "@{author_name} failed to edit {sticker_name:?} in {guild_name}: {e:?}"
            );

            Err(format!("An error occurred while editing `{sticker_name}`."))
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(msg) => builders::replies::build_error_reply_with_embed(msg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
