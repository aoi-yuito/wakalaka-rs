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

    let user = if let Some(name) = queries::users::select_lastfm_name(db, &author_id).await? {
        name
    } else {
        let reply =
            components::replies::error_reply_embed("Your Last.fm account must be linked!", true);

        ctx.send(reply).await?;

        return Ok(());
    };
    let sk = if let Some(session_key) = queries::users::select_lastfm_key(db, &author_id).await? {
        session_key
    } else {
        let reply =
            components::replies::error_reply_embed("Your Last.fm account must be linked!", true);

        ctx.send(reply).await?;

        return Ok(());
    };

    let json = integrations::lastfm::artist::get_tags(artist, None::<String>, &user, None).await?;
    let tags = match &json["tags"]["tag"] {
        serde_json::Value::Array(tags) => tags,
        _ => {
            let reply = components::replies::error_reply_embed(
                format!("Cannot find tags for **{artist}** assigned by **{user}**.",),
                true,
            );

            ctx.send(reply).await?;

            return Ok(());
        }
    };

    let artist_tags = tags
        .iter()
        .map(|tag| {
            format!(
                "{}",
                tag["name"].as_str().expect("tag.name is not a string")
            )
        })
        .collect::<Vec<_>>();

    let tag_matches = artist_tags
        .iter()
        .filter(|&t| t.to_lowercase() == tag.to_lowercase())
        .collect::<Vec<_>>();
    if tag_matches.is_empty() {
        let reply = components::replies::error_reply_embed(
            format!("Cannot find `{tag}` for **{artist}** assigned by **{user}**.",),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let result = match integrations::lastfm::artist::remove_tag(artist, tag, sk).await {
        Ok(_) => Ok(format!("`{tag}` has been removed from **{artist}**.")),
        Err(why) => {
            error!("Failed to remove tag from artist: {:?}", why);

            Err(format!(
                "An error occurred while removing tag from **{artist}**."
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
