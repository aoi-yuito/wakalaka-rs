// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use chrono::NaiveDateTime;
use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter, Timestamp};

use crate::{
    framework::options::commands::music::lastfm::LASTFM_COLOUR, integrations, utils, Context,
    Throwable,
};

struct Info {
    name: String,
    mbid: String,
    url: String,
    image_xl: String,
    streamable: String,
    listeners: String,
    playcount: String,
    userplaycount: String,
    tags: Vec<(String, String)>,
    summary: String,
    published: Timestamp,
}

#[poise::command(
    slash_command,
    category = "Music",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5,
    ephemeral
)]
/// Get information about an artist.
pub(super) async fn getinfo(
    ctx: Context<'_>,
    #[description = "The name of the artist."] artist: String,
    #[description = "The musicbrainz ID for the artist."]
    #[min_length = 36]
    #[max_length = 36]
    mbid: Option<String>,
    #[description = "The language to return the biography in. (ISO 639-2)"]
    #[min_length = 3]
    #[max_length = 3]
    lang: Option<String>,
    #[description = "Whether to autocorrect the artist name."] autocorrect: Option<bool>,
    #[description = "The username to look up."] username: Option<String>,
) -> Throwable<()> {
    let artist = artist.trim();

    let username = match username {
        Some(username) => username.clone(),
        None => {
            let lastfm_name = if let Some(name) =
                crate::database::queries::users::select_lastfm_name(
                    &ctx.data().db,
                    &ctx.author().id,
                )
                .await?
            {
                name
            } else {
                let reply = CreateReply::default()
                    .content("Your Last.fm account must be linked!")
                    .ephemeral(true);

                ctx.send(reply).await?;

                return Ok(());
            };
            lastfm_name
        }
    };

    let json =
        integrations::lastfm::artist::get_info(artist, mbid, lang, autocorrect, username).await?;

    let artist = &json["artist"];
    let artist_stats = &artist["stats"];
    let artist_tags = &artist["tags"]["tag"];
    let artist_bio = &artist["bio"];

    let get_info = Info {
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
        image_xl: format!(
            "{}",
            artist["image"][3]["#text"]
                .as_str()
                .expect("artist.image is not a string")
        ),
        streamable: if artist["streamable"]
            .as_str()
            .expect("artist.streamable is not a string")
            == "1"
        {
            format!("✅")
        } else {
            format!("❌")
        },
        listeners: format!(
            "{}",
            artist_stats["listeners"]
                .as_str()
                .expect("artist.stats.listeners is not a string")
        ),
        playcount: format!(
            "{}",
            artist_stats["playcount"]
                .as_str()
                .expect("artist.stats.playcount is not a string")
        ),
        userplaycount: format!(
            "{}",
            artist_stats["userplaycount"]
                .as_str()
                .expect("artist.stats.userplaycount is not a string")
        ),
        tags: artist_tags
            .as_array()
            .expect("artist.tags.tag is not an array")
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
            .collect(),
        summary: format!(
            "{}",
            artist_bio["summary"]
                .as_str()
                .expect("artist.bio.summary is not a string")
        ),
        published: {
            let published = artist_bio["published"]
                .as_str()
                .expect("artist.bio.published is not a string");

            let datetime = NaiveDateTime::parse_from_str(&published, "%d %b %Y, %H:%M")
                .expect("artist.bio.published is not a valid timestamp")
                .and_utc();

            Timestamp::from(datetime)
        },
    };

    let artist_name = &get_info.name;
    let artist_mbid = &get_info.mbid;
    let artist_url = &get_info.url;
    let artist_image_xl = &get_info.image_xl;
    let artist_streamable = &get_info.streamable;
    let artist_listener_count = &get_info.listeners;
    let artist_plays_count = &format!("{} ({})", &get_info.playcount, &get_info.userplaycount);
    let artist_tags = &get_info.tags;
    let artist_summary = utils::html_to_md(get_info.summary).replace(" [", "\n\n[");
    let artist_published = &get_info.published;

    let embed_description = format!(
        "{artist_summary}\n\n{}",
        artist_tags
            .iter()
            .map(|(tag, tag_url)| format!("[{tag}]({tag_url})"))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let embed_fields = vec![
        ("👂 Listeners", artist_listener_count, true),
        ("🎵 Plays", artist_plays_count, true),
        ("🎧 Streamable", artist_streamable, false),
    ];
    let embed_footer = CreateEmbedFooter::new(artist_mbid);

    let embed = CreateEmbed::default()
        .title(artist_name)
        .url(artist_url)
        .description(embed_description)
        .thumbnail(artist_image_xl)
        .fields(embed_fields)
        .footer(embed_footer)
        .timestamp(artist_published)
        .colour(LASTFM_COLOUR);

    let reply = CreateReply::default().embed(embed).ephemeral(false);

    ctx.send(reply).await?;

    Ok(())
}
