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

struct TopAlbums {
    name: String,
    mbid: String,
    playcount: String,
    url: String,
    image_xl: String,
}

#[poise::command(
    slash_command,
    category = "Music",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    user_cooldown = 5,
    ephemeral
)]
/// Get the top albums for an artist.
pub(super) async fn gettopalbums(
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
    #[description = "The number of albums to display."]
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

    let json = integrations::lastfm::artist::get_top_albums(artist, mbid, autocorrect, limit, page)
        .await?;

    let albums = json["topalbums"]["album"]
        .as_array()
        .expect("topalbums.album is not an array");

    let album_count = albums.len();

    let top_albums = albums
        .iter()
        .map(|album| TopAlbums {
            name: format!(
                "{}",
                album["name"].as_str().expect("album.name is not a string")
            ),
            mbid: format!("{}", album["mbid"].as_str().unwrap_or("")),
            playcount: format!(
                "{}",
                album["playcount"]
                    .as_i64()
                    .expect("album.playcount is not an integer")
            ),
            url: format!(
                "{}",
                album["url"].as_str().expect("album.url is not a string")
            ),
            image_xl: format!(
                "{}",
                album["image"][3]["#text"]
                    .as_str()
                    .expect("album.image[3].#text is not a string")
            ),
        })
        .collect::<Vec<_>>();

    let mut album_name = &top_albums[0].name;
    let mut album_mbid = &top_albums[0].mbid;
    let mut album_plays_count = &top_albums[0].playcount;
    let mut album_url = &top_albums[0].url;
    let mut album_image_xl = &top_albums[0].image_xl;

    let mut embed_author = CreateEmbedAuthor::new(artist);
    let mut embed_fields = vec![("🎵 Plays", album_plays_count, true)];
    let mut embed_footer = CreateEmbedFooter::new(album_mbid);

    let mut embed = CreateEmbed::default()
        .author(embed_author)
        .title(album_name)
        .url(album_url)
        .thumbnail(album_image_xl)
        .fields(embed_fields)
        .footer(embed_footer)
        .colour(LASTFM_COLOUR);

    let mut first_page = builders::buttons::first_button(true);
    let mut previous_page = builders::buttons::previous_button(true);
    let mut next_page = builders::buttons::next_button(album_count < 1);
    let mut last_page = builders::buttons::last_button(album_count < 1);

    let mut action_rows = vec![CreateActionRow::Buttons(vec![
        first_page,
        previous_page,
        next_page,
        last_page,
    ])];

    let mut current_album_index = 0;
    let mut current_album_page = format!("{} of {album_count}", current_album_index + 1);

    let reply = CreateReply::default()
        .content(current_album_page)
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

            current_album_index = match component_id {
                "first_page" => 0,
                "last_page" => album_count - 1,
                "previous_page" => {
                    if current_album_index > 0 {
                        current_album_index - 1
                    } else {
                        album_count - 1
                    }
                }
                "next_page" => {
                    if current_album_index < album_count - 1 {
                        current_album_index + 1
                    } else {
                        0
                    }
                }
                _ => current_album_index,
            };

            first_page = builders::buttons::first_button(current_album_index == 0);
            previous_page = builders::buttons::previous_button(current_album_index == 0);
            next_page =
                builders::buttons::next_button(current_album_index == album_count - 1);
            last_page =
                builders::buttons::last_button(current_album_index == album_count - 1);

            action_rows = vec![CreateActionRow::Buttons(vec![
                first_page,
                previous_page,
                next_page,
                last_page,
            ])];

            match component_id {
                "first_page" | "last_page" | "previous_page" | "next_page" => {
                    current_album_page = format!("{} of {album_count}", current_album_index + 1);

                    album_name = &top_albums[current_album_index].name;
                    album_mbid = &top_albums[current_album_index].mbid;
                    album_plays_count = &top_albums[current_album_index].playcount;
                    album_url = &top_albums[current_album_index].url;
                    album_image_xl = &top_albums[current_album_index].image_xl;

                    embed_author = CreateEmbedAuthor::new(artist);
                    embed_fields = vec![("🎵 Plays", album_plays_count, true)];
                    embed_footer = CreateEmbedFooter::new(album_mbid);

                    embed = CreateEmbed::default()
                        .author(embed_author)
                        .title(album_name)
                        .url(album_url)
                        .thumbnail(album_image_xl)
                        .fields(embed_fields)
                        .footer(embed_footer)
                        .colour(LASTFM_COLOUR);

                    let response_message = CreateInteractionResponseMessage::new()
                        .content(current_album_page)
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
