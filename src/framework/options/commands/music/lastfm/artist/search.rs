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
    framework::options::commands::music::lastfm::LASTFM_COLOUR, integrations, utils::builders,
    Context, Throwable,
};

struct Search {
    name: String,
    listeners: String,
    mbid: String,
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
/// Search for an artist.
pub(super) async fn search(
    ctx: Context<'_>,
    #[description = "The artist name."]
    #[min_length = 2]
    #[max_length = 15]
    artist: String,
    #[description = "The number of artists to display."]
    #[min = 1]
    #[max = 30]
    limit: Option<u8>,
    #[min = 1]
    #[description = "The page to display."]
    page: Option<u8>,
) -> Throwable<()> {
    let artist_re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$")?;
    let artist = artist.trim();
    if !artist_re.is_match(artist) {
        let reply = builders::replies::error_reply_embed("Name of the artist must begin with a letter and contain only letters, numbers, hyphens, and underscores!", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let json = integrations::lastfm::artist::search(artist, limit, page).await?;

    let artists = json["results"]["artistmatches"]["artist"]
        .as_array()
        .expect("artistmatches.artist is not an array");

    let artist_count = artists.len();

    let search = artists
        .iter()
        .map(|artist| Search {
            name: format!(
                "{}",
                artist["name"]
                    .as_str()
                    .expect("artist.name is not a string")
            ),
            listeners: format!(
                "{}",
                artist["listeners"]
                    .as_str()
                    .expect("artist.listeners is not a string")
            ),
            mbid: format!("{}", artist["mbid"].as_str().unwrap_or("")),
            url: format!(
                "{}",
                artist["url"].as_str().expect("artist.url is not a string")
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
            image_xl: format!(
                "{}",
                artist["image"][3]["#text"]
                    .as_str()
                    .expect("artist.image is not a string")
            ),
        })
        .collect::<Vec<_>>();

    let mut artist_name = &search[0].name;
    let mut artist_listener_count = &search[0].listeners;
    let mut artist_mbid = &search[0].mbid;
    let mut artist_url = &search[0].url;
    let mut artist_streamable = &search[0].streamable;
    let mut artist_image_xl = &search[0].image_xl;

    let mut embed_fields = vec![
        ("👂 Listeners", artist_listener_count, false),
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

    let mut first_page = builders::buttons::first_button(true);
    let mut previous_page = builders::buttons::previous_button(true);
    let mut next_page = builders::buttons::next_button(artist_count < 1);
    let mut last_page = builders::buttons::last_button(artist_count < 1);

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

            first_page = builders::buttons::first_button(current_artist_index == 0);
            previous_page = builders::buttons::previous_button(current_artist_index == 0);
            next_page =
                builders::buttons::next_button(current_artist_index == artist_count - 1);
            last_page =
                builders::buttons::last_button(current_artist_index == artist_count - 1);

            action_rows = vec![CreateActionRow::Buttons(vec![
                first_page,
                previous_page,
                next_page,
                last_page,
            ])];

            match component_id {
                "first_page" | "last_page" | "previous_page" | "next_page" => {
                    current_artist_page = format!("{} of {artist_count}", current_artist_index + 1);

                    artist_name = &search[current_artist_index].name;
                    artist_listener_count = &search[current_artist_index].listeners;
                    artist_mbid = &search[current_artist_index].mbid;
                    artist_url = &search[current_artist_index].url;
                    artist_streamable = &search[current_artist_index].streamable;
                    artist_image_xl = &search[current_artist_index].image_xl;

                    embed_fields = vec![
                        ("👂 Listeners", artist_listener_count, false),
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
                let reply = builders::replies::error_reply_embed(message, true);

                ctx.send(reply).await?;
                break;
            }
        }
    }

    Ok(())
}
