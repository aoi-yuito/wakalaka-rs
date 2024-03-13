// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::time::Duration;

use poise::CreateReply;
use regex::Regex;
use serenity::all::{
    CreateActionRow, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use tokio::time::Instant;

use crate::{
    framework::options::commands::music::lastfm::LASTFM_COLOUR, integrations, utils::builders,
    Context, Throwable,
};

struct TopTracks {
    name: String,
    mbid: String,
    playcount: String,
    listeners: String,
    url: String,
    streamable: String,
    image_xl: String,
}

#[poise::command(
    slash_command,
    category = "Music",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5,
    ephemeral
)]
/// Get the top tracks by an artist.
pub(super) async fn gettoptracks(
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
    #[min = 1]
    #[description = "The page to display."]
    page: Option<u8>,
    #[description = "The number of artists to display."]
    #[min = 1]
    #[max = 50]
    limit: Option<u8>,
) -> Throwable<()> {
    let artist_re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$")?;
    let artist = artist.trim();
    if !artist_re.is_match(artist) {
        let reply = builders::replies::error_reply_embed("Name of the artist must begin with a letter and contain only letters, numbers, hyphens, and underscores!", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let json = integrations::lastfm::artist::get_top_tracks(artist, mbid, autocorrect, limit, page)
        .await?;

    let tracks = json["toptracks"]["track"]
        .as_array()
        .expect("toptracks.track is not an array");

    let track_count = tracks.len();

    let top_tracks = tracks
        .iter()
        .map(|track| TopTracks {
            name: format!(
                "{}",
                track["name"].as_str().expect("track.name is not a string")
            ),
            mbid: format!("{}", track["mbid"].as_str().unwrap_or("")),
            playcount: format!(
                "{}",
                track["playcount"]
                    .as_str()
                    .expect("track.playcount is not a string")
            ),
            listeners: format!(
                "{}",
                track["listeners"]
                    .as_str()
                    .expect("track.listeners is not a string")
            ),
            url: format!(
                "{}",
                track["url"].as_str().expect("track.url is not a string")
            ),
            streamable: if track["streamable"]
                .as_str()
                .expect("track.streamable is not a string")
                == "1"
            {
                format!("✅")
            } else {
                format!("❌")
            },
            image_xl: format!(
                "{}",
                track["image"][3]["#text"]
                    .as_str()
                    .expect("track.image is not a string")
            ),
        })
        .collect::<Vec<_>>();

    let mut track_name = &top_tracks[0].name;
    let mut track_mbid = &top_tracks[0].mbid;
    let mut track_plays_count = &top_tracks[0].playcount;
    let mut track_listener_count = &top_tracks[0].listeners;
    let mut track_url = &top_tracks[0].url;
    let mut track_streamable = &top_tracks[0].streamable;
    let mut track_image_xl = &top_tracks[0].image_xl;

    let mut embed_author = CreateEmbedAuthor::new(artist);
    let mut embed_fields = vec![
        ("🎵 Plays", track_plays_count, true),
        ("👂 Listeners", track_listener_count, true),
        ("🎧 Streamable", track_streamable, false),
    ];
    let mut embed_footer = CreateEmbedFooter::new(track_mbid);

    let mut embed = CreateEmbed::default()
        .author(embed_author)
        .title(track_name)
        .url(track_url)
        .thumbnail(track_image_xl)
        .fields(embed_fields)
        .footer(embed_footer)
        .colour(LASTFM_COLOUR);

    let mut first_page = builders::buttons::first_button(true);
    let mut previous_page = builders::buttons::previous_button(true);
    let mut next_page = builders::buttons::next_button(track_count < 1);
    let mut last_page = builders::buttons::last_button(track_count < 1);

    let mut action_rows = vec![CreateActionRow::Buttons(vec![
        first_page,
        previous_page,
        next_page,
        last_page,
    ])];

    let mut current_track_index = 0;
    let mut current_track_page = format!("{} of {track_count}", current_track_index + 1);

    let reply = CreateReply::default()
        .content(current_track_page)
        .embed(embed)
        .components(action_rows)
        .ephemeral(false);

    let message = ctx.send(reply).await?.into_message().await?;

    let duration = Duration::from_secs(60 * 3);

    let start_time = Instant::now();
    let elapsed_time = start_time.elapsed();
    while elapsed_time < duration {
        let interaction_collector = message.await_component_interactions(ctx);

        let result = if let Ok(Some(interaction)) =
            tokio::time::timeout(duration, interaction_collector.next()).await
        {
            let component_id = interaction.data.custom_id.as_str();

            current_track_index = match component_id {
                "first_page" => 0,
                "last_page" => track_count - 1,
                "previous_page" => {
                    if current_track_index > 0 {
                        current_track_index - 1
                    } else {
                        track_count - 1
                    }
                }
                "next_page" => {
                    if current_track_index < track_count - 1 {
                        current_track_index + 1
                    } else {
                        0
                    }
                }
                _ => current_track_index,
            };

            first_page = builders::buttons::first_button(current_track_index == 0);
            previous_page = builders::buttons::previous_button(current_track_index == 0);
            next_page =
                builders::buttons::next_button(current_track_index == track_count - 1);
            last_page =
                builders::buttons::last_button(current_track_index == track_count - 1);

            action_rows = vec![CreateActionRow::Buttons(vec![
                first_page,
                previous_page,
                next_page,
                last_page,
            ])];

            match component_id {
                "first_page" | "last_page" | "previous_page" | "next_page" => {
                    current_track_page = format!("{} of {track_count}", current_track_index + 1);

                    track_name = &top_tracks[current_track_index].name;
                    track_mbid = &top_tracks[current_track_index].mbid;
                    track_plays_count = &top_tracks[current_track_index].playcount;
                    track_listener_count = &top_tracks[current_track_index].listeners;
                    track_url = &top_tracks[current_track_index].url;
                    track_streamable = &top_tracks[current_track_index].streamable;
                    track_image_xl = &top_tracks[current_track_index].image_xl;

                    embed_author = CreateEmbedAuthor::new(artist);
                    embed_fields = vec![
                        ("🎵 Plays", track_plays_count, true),
                        ("👂 Listeners", track_listener_count, true),
                        ("🎧 Streamable", track_streamable, false),
                    ];
                    embed_footer = CreateEmbedFooter::new(track_mbid);

                    embed = CreateEmbed::default()
                        .author(embed_author)
                        .title(track_name)
                        .url(track_url)
                        .thumbnail(track_image_xl)
                        .fields(embed_fields)
                        .footer(embed_footer)
                        .colour(LASTFM_COLOUR);

                    let response_message = CreateInteractionResponseMessage::new()
                        .content(current_track_page)
                        .embed(embed)
                        .components(action_rows);
                    let response = CreateInteractionResponse::UpdateMessage(response_message);

                    Ok(interaction.create_response(ctx, response).await?)
                }
                _ => Err(format!(
                    "Missing interaction for the following: `{component_id}`"
                )),
            }
        } else {
            message.delete(ctx).await?;
            break;
        };
        match result {
            Ok(_) => {}
            Err(message) => {
                let reply = builders::replies::error_reply_embed(message, true);

                ctx.send(reply).await?;
                break;
            }
        }
    }

    Ok(())
}
