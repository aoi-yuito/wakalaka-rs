// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use regex::Regex;
use serenity::all::CreateEmbed;

use crate::{
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
/// Get the most popular tags for an artist.
pub(super) async fn gettoptags(
    ctx: Context<'_>,
    #[description = "The artist name."]
    #[min_length = 2]
    #[max_length = 15]
    artist: String,
    #[description = "The musicbrainz ID for the artist."]
    #[min_length = 36]
    #[max_length = 36]
    mbid: Option<String>,
    #[description = "Whether to autocorrect the artist name."] autocorrect: Option<bool>,
) -> Throwable<()> {
    let artist_re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$")?;
    let artist = artist.trim();
    if !artist_re.is_match(artist) {
        let reply = components::replies::error_reply_embed("Name of the artist must begin with a letter and contain only letters, numbers, hyphens, and underscores!", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let json = integrations::lastfm::artist::get_top_tags(artist, mbid, autocorrect).await?;

    let toptags = json["toptags"]
        .as_object()
        .expect("toptags is not an object");
    let tags = toptags["tag"]
        .as_array()
        .expect("toptags.tag is not an array");

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

    let embed_description = format!(
        "{}",
        artist_tags
            .iter()
            .map(|(tag, tag_url)| format!("[{tag}]({tag_url})"))
            .collect::<Vec<_>>()
            .join(" ")
    );

    let embed = CreateEmbed::default()
        .title(artist)
        .url(artist_url)
        .description(embed_description)
        .colour(LASTFM_COLOUR);

    let reply = CreateReply::default().embed(embed).ephemeral(false);

    ctx.send(reply).await?;

    Ok(())
}
