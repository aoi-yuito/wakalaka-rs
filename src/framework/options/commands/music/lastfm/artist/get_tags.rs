// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedAuthor};

use crate::{
    database::queries,
    framework::options::commands::music::lastfm::{LASTFM_COLOUR, MUSIC_URL},
    integrations,
    utils::components,
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Music",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5,
    ephemeral
)]
/// Get user-assigned tags for an artist.
pub(super) async fn gettags(
    ctx: Context<'_>,
    #[description = "The name of the artist."] artist: String,
    #[description = "The musicbrainz ID for the artist."]
    #[min_length = 36]
    #[max_length = 36]
    mbid: Option<String>,
    #[description = "The user to look up."] user: Option<String>,
    #[description = "Whether to autocorrect the artist name."] autocorrect: Option<bool>,
) -> Throwable<()> {
    let db = &ctx.data().db;

    let artist = artist.trim();

    let author = ctx.author();
    let author_id = author.id;

    let user = match user {
        Some(user) => user.clone(),
        None => {
            let lastfm_name =
                if let Some(name) = queries::users::select_lastfm_name(db, &author_id).await? {
                    name
                } else {
                    let reply = components::replies::error_reply_embed(
                        "Your Last.fm account must be linked!",
                        true,
                    );

                    ctx.send(reply).await?;

                    return Ok(());
                };
            lastfm_name
        }
    };

    let json = integrations::lastfm::artist::get_tags(artist, mbid, &user, autocorrect).await?;

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

    let artist_url = format!("{MUSIC_URL}{}", artist.replace(" ", "+"));
    let artist_tags = tags
        .iter()
        .map(|tag| {
            (
                format!(
                    "#{}",
                    tag["name"].as_str().expect("tag.name is not a string")
                )
                .replace(" ", "_"),
                format!("{}", tag["url"].as_str().expect("tag.url is not a string")),
            )
        })
        .collect::<Vec<(_, _)>>();

    let embed_author = CreateEmbedAuthor::new(user);
    let embed_description = format!(
        "{}",
        artist_tags
            .iter()
            .map(|(tag, tag_url)| format!("[{tag}]({tag_url})"))
            .collect::<Vec<_>>()
            .join(" ")
    );

    let embed = CreateEmbed::default()
        .author(embed_author)
        .title(artist)
        .url(artist_url)
        .description(embed_description)
        .colour(LASTFM_COLOUR);

    let reply = CreateReply::default().embed(embed).ephemeral(false);

    ctx.send(reply).await?;

    Ok(())
}
