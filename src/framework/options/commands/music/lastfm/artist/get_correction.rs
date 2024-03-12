// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use regex::Regex;
use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::{
    framework::options::commands::music::lastfm::LASTFM_COLOUR, integrations, utils::components,
    Context, Throwable,
};

struct Correction {
    name: String,
    mbid: String,
    url: String,
}

#[poise::command(
    slash_command,
    category = "Music",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5,
    ephemeral
)]
/// Get the corrected artist name.
pub(super) async fn getcorrection(
    ctx: Context<'_>,
    #[description = "The artist name."]
    #[min_length = 2]
    #[max_length = 15]
    artist: String,
) -> Throwable<()> {
    let artist_re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$")?;
    let artist = artist.trim();
    if !artist_re.is_match(artist) {
        let reply = components::replies::error_reply_embed("Name of the artist must begin with a letter and contain only letters, numbers, hyphens, and underscores!", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let json = integrations::lastfm::artist::get_correction(artist).await?;

    let corrections = json["corrections"]
        .as_object()
        .expect("corrections is not an object");
    let correction = corrections["correction"]
        .as_object()
        .expect("corrections.correction is not an object");

    let artist = &correction["artist"];

    let get_correction = Correction {
        name: format!(
            "{}",
            artist["name"]
                .as_str()
                .expect("artist.name is not a string")
        ),
        mbid: format!("{}", artist["mbid"].as_str().unwrap_or("")),
        url: format!(
            "{}",
            artist["url"].as_str().expect("artist.url is not a string")
        ),
    };

    let artist_name = &get_correction.name;
    let artist_mbid = &get_correction.mbid;
    let artist_url = &get_correction.url;

    let embed_footer = CreateEmbedFooter::new(artist_mbid);

    let embed = CreateEmbed::default()
        .title(artist_name)
        .url(artist_url)
        .footer(embed_footer)
        .colour(LASTFM_COLOUR);

    let reply = CreateReply::default().embed(embed).ephemeral(false);

    ctx.send(reply).await?;

    Ok(())
}
