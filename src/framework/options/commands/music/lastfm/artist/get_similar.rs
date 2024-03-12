// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::time::Duration;

use poise::CreateReply;
use regex::Regex;
use serenity::all::{
    CreateActionRow, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use tokio::time::Instant;

use crate::{
    framework::options::commands::music::lastfm::LASTFM_COLOUR, integrations, utils::components,
    Context, Throwable,
};

struct Similar {
    name: String,
    mbid: String,
    match_: String,
    url: String,
    image_xl: String,
    streamable: String,
}

#[poise::command(
    slash_command,
    category = "Music",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5,
    ephemeral
)]
/// Get similar artists for the given artist.
pub(super) async fn getsimilar(
    ctx: Context<'_>,
    #[description = "The artist name."]
    #[min_length = 2]
    #[max_length = 15]
    artist: String,
    #[description = "The amount of similar artists to display."]
    #[min = 1]
    #[max = 100]
    limit: Option<u8>,
    #[description = "Whether to autocorrect the artist name."] autocorrect: Option<bool>,
    #[description = "The musicbrainz ID for the artist."]
    #[min_length = 36]
    #[max_length = 36]
    mbid: Option<String>,
) -> Throwable<()> {
    let artist_re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$")?;
    let artist = artist.trim();
    if !artist_re.is_match(artist) {
        let reply = components::replies::error_reply_embed("Name of the artist must begin with a letter and contain only letters, numbers, hyphens, and underscores!", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let json = integrations::lastfm::artist::get_similar(artist, limit, autocorrect, mbid).await?;

    let similar_artists = json["similarartists"]
        .as_object()
        .expect("similarartists is not an object");
    let artists = similar_artists["artist"]
        .as_array()
        .expect("similarartists.artist is not an array");

    let artist_count = artists.len();

    let get_similar = artists
        .iter()
        .map(|artist| Similar {
            name: format!(
                "{}",
                artist["name"]
                    .as_str()
                    .expect("artist.name is not a string")
            ),
            mbid: format!("{}", artist["mbid"].as_str().unwrap_or("")),
            match_: format!(
                "{:.2}%",
                artist["match"]
                    .as_str()
                    .expect("artist.match is not a string")
                    .parse::<f32>()
                    .expect("artist.match is not a float")
                    * 100.0
            ),
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
        })
        .collect::<Vec<_>>();

    let mut artist_name = &get_similar[0].name;
    let mut artist_mbid = &get_similar[0].mbid;
    let mut artist_match = &get_similar[0].match_;
    let mut artist_url = &get_similar[0].url;
    let mut artist_image_xl = &get_similar[0].image_xl;
    let mut artist_streamable = &get_similar[0].streamable;

    let mut embed_fields = vec![
        ("🤝 Match", artist_match, false),
        ("🎧 Streamable", artist_streamable, false),
    ];
    let mut embed_footer = CreateEmbedFooter::new(artist_mbid);

    let mut embed = CreateEmbed::default()
        .title(artist_name)
        .url(artist_url)
        .thumbnail(artist_image_xl)
        .fields(embed_fields)
        .footer(embed_footer)
        .colour(LASTFM_COLOUR);

    let mut first_page = components::buttons::first_page_button(true);
    let mut previous_page = components::buttons::previous_page_button(true);
    let mut next_page = components::buttons::next_page_button(artist_count < 1);
    let mut last_page = components::buttons::last_page_button(artist_count < 1);

    let mut action_rows = vec![CreateActionRow::Buttons(vec![
        first_page,
        previous_page,
        next_page,
        last_page,
    ])];

    let mut current_artist_index = 0;
    let mut current_artist_page = format!("{} of {artist_count}", current_artist_index + 1);

    let reply = CreateReply::default()
        .content(current_artist_page)
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

            current_artist_index = match component_id {
                "first_page" => 0,
                "last_page" => artist_count - 1,
                "previous_page" => {
                    if current_artist_index > 0 {
                        current_artist_index - 1
                    } else {
                        artist_count - 1
                    }
                }
                "next_page" => {
                    if current_artist_index < artist_count - 1 {
                        current_artist_index + 1
                    } else {
                        0
                    }
                }
                _ => current_artist_index,
            };

            first_page = components::buttons::first_page_button(current_artist_index == 0);
            previous_page = components::buttons::previous_page_button(current_artist_index == 0);
            next_page =
                components::buttons::next_page_button(current_artist_index == artist_count - 1);
            last_page =
                components::buttons::last_page_button(current_artist_index == artist_count - 1);

            action_rows = vec![CreateActionRow::Buttons(vec![
                first_page,
                previous_page,
                next_page,
                last_page,
            ])];

            match component_id {
                "first_page" | "last_page" | "previous_page" | "next_page" => {
                    current_artist_page = format!("{} of {artist_count}", current_artist_index + 1);

                    artist_name = &get_similar[current_artist_index].name;
                    artist_mbid = &get_similar[current_artist_index].mbid;
                    artist_match = &get_similar[current_artist_index].match_;
                    artist_url = &get_similar[current_artist_index].url;
                    artist_image_xl = &get_similar[current_artist_index].image_xl;
                    artist_streamable = &get_similar[current_artist_index].streamable;

                    embed_fields = vec![
                        ("🤝 Match", artist_match, false),
                        ("🎧 Streamable", artist_streamable, false),
                    ];
                    embed_footer = CreateEmbedFooter::new(artist_mbid);

                    embed = CreateEmbed::default()
                        .title(artist_name)
                        .url(artist_url)
                        .thumbnail(artist_image_xl)
                        .fields(embed_fields)
                        .footer(embed_footer)
                        .colour(LASTFM_COLOUR);

                    let response_message = CreateInteractionResponseMessage::new()
                        .content(current_artist_page)
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
                let reply = components::replies::error_reply_embed(message, true);

                ctx.send(reply).await?;
                break;
            }
        }
    }

    Ok(())
}
