// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use regex::Regex;
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
    #[description = "The artist name."]
    #[min_length = 2]
    #[max_length = 15]
    artist: String,
    #[description = "The tags to add."] tags: String,
) -> Throwable<()> {
    let db = &ctx.data().db;

    let artist_re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$")?;
    let artist = artist.trim();
    if !artist_re.is_match(artist) {
        let reply = components::replies::error_reply_embed("Name of the artist must begin with a letter and contain only letters, numbers, hyphens, and underscores!", true);

        ctx.send(reply).await?;

        return Ok(());
    }

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
    let json_tags = match &json["tags"]["tag"] {
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
    let artist_tags = json_tags
        .iter()
        .map(|tag| {
            format!(
                "{}",
                tag["name"].as_str().expect("tag.name is not a string")
            )
        })
        .collect::<Vec<_>>();

    let tag_matches = tags
        .split(',')
        .map(|tag| tag.trim())
        .filter(|tag| artist_tags.contains(&format!("{tag}")))
        .collect::<Vec<_>>();
    if !tag_matches.is_empty() {
        let reply = components::replies::error_reply_embed(
            format!("`{tags}` already exists for **{artist}**!",),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let result = match integrations::lastfm::artist::add_tags(artist, tags, sk).await {
        Ok(_) => {
            if tag_count == 1 {
                Ok(format!("`{tags}` has been added to **{artist}**."))
            } else {
                Ok(format!("`{tags}` have been added to **{artist}**."))
            }
        }
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
