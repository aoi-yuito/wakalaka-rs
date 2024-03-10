// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use tracing::error;

use crate::{database::queries, integrations, utils::components, Context, Throwable};

#[poise::command(
    slash_command,
    category = "Music",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5,
    ephemeral
)]
/// Tag an artist with your own supplied tags.
pub(super) async fn addtags(
    ctx: Context<'_>,
    #[description = "The name of the artist."] artist: String,
    #[description = "The tags to add."] tags: String,
) -> Throwable<()> {
    let db = &ctx.data().db;

    let artist = artist.trim();

    let tags = &tags
        .split(',')
        .map(|tag| tag.trim())
        .collect::<Vec<_>>()
        .join(",");

    let tag_count = tags.matches(',').count() + 1;
    if tag_count > 10 || tag_count < 1 {
        let reply = components::replies::error_reply_embed(
            "Number of tags must be between 1 and 10, separated by commas!",
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let author = ctx.author();
    let author_id = author.id;

    let sk = if let Some(session_key) = queries::users::select_lastfm_key(db, &author_id).await? {
        session_key
    } else {
        let reply =
            components::replies::error_reply_embed("Your Last.fm account must be linked!", true);

        ctx.send(reply).await?;

        return Ok(());
    };

    let result = match integrations::lastfm::artist::add_tags(artist, tags, sk).await {
        Ok(_) => Ok(format!("`{tags}` have been added to **{artist}**.")),
        Err(why) => {
            error!("Failed to add tags to artist: {why:?}");

            Err(format!(
                "An error occurred while adding tags to **{artist}**."
            ))
        }
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
