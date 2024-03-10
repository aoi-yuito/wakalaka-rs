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
/// Remove a supplied tag from an artist.
pub(super) async fn removetag(
    ctx: Context<'_>,
    #[description = "The name of the artist."] artist: String,
    #[description = "The tag to remove."] tag: String,
) -> Throwable<()> {
    let db = &ctx.data().db;

    let artist = artist.trim();
    let tag = tag.trim();

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

    let result = match integrations::lastfm::artist::remove_tag(artist, tag, sk).await {
        Ok(_) => Ok(format!("`{tag}` has been removed from **{artist}**.")),
        Err(e) => {
            error!("Failed to remove tag: {:?}", e);

            Err("Failed to remove tag.".to_string())
        }
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
