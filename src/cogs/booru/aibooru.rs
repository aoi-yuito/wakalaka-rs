/**
 * Copyright (C) 2024 Kasutaja
 *
 * wakalaka-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * wakalaka-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::uses::*;

struct AiMetadata {
    media_metadata: String,
    exif_ifd_user_comment: Option<String>,
    file_metadata: metadata::FileMetadata,
}

impl AiMetadata {
    fn extract_media_metadata_jpeg(response: &serde_json::Value) -> Self {
        let media_metadata = response["media_metadata"].to_string();
        let metadata = response[media_metadata.clone()]["metadata"].to_string();

        let exif_ifd_user_comment = response[metadata]["ExifIFD:UserComment"].to_string();
        dbg!(&exif_ifd_user_comment);

        // let prompt = exif_ifd_user_comment_split[0].to_string();
        // let negative_prompt = exif_ifd_user_comment_split[1].to_string();
        // let sampler = exif_ifd_user_comment_split[3].to_string();
        // let seed = exif_ifd_user_comment_split[5].to_string();
        // let steps = exif_ifd_user_comment_split[2].to_string();
        // let cfg_scale = exif_ifd_user_comment_split[4].to_string();
        // let model = exif_ifd_user_comment_split[7].to_string();
        // let model_hash = exif_ifd_user_comment_split[6].to_string().to_uppercase();
        // let vae = exif_ifd_user_comment_split[9].to_string();
        // let vae_hash = exif_ifd_user_comment_split[8].to_string().to_uppercase();

        let mut file_metadata = metadata::FileMetadata::default();
        // file_metadata.prompt = prompt;
        // file_metadata.negative_prompt = negative_prompt;
        // file_metadata.sampler = sampler;
        // file_metadata.seed = seed.parse::<u32>().unwrap();
        // file_metadata.steps = steps.parse::<u32>().unwrap();
        // file_metadata.cfg_scale = cfg_scale.parse::<f32>().unwrap();
        // file_metadata.model_hash = format!("{model} {model_hash}");
        // file_metadata.vae_hash = Some(format!("{vae} {vae_hash}"));

        Self {
            media_metadata,
            exif_ifd_user_comment: Some(exif_ifd_user_comment),
            file_metadata,
        }
    }

    fn extract_media_metadata_png(response: &serde_json::Value) -> Self {
        let media_metadata = response["media_metadata"].to_string();
        let metadata = response[media_metadata.clone()]["metadata"].to_string();

        let mut file_metadata = metadata::FileMetadata::default();

        let ai = metadata::ArtificialIntelligence::new_png(metadata.clone());
        match ai {
            metadata::ArtificialIntelligence::A1111 => {
                todo!()
            }
            metadata::ArtificialIntelligence::NovelAI => {
                let png_source = response[metadata.clone()]["PNG:Source"].to_string();
                let png_comment = response[metadata.clone()]["PNG:Comment"].to_string();
                let png_description = response[metadata]["PNG:Description"].to_string();

                let prompt = png_comment.split("prompt").collect::<Vec<&str>>();
                let steps = png_comment.split("steps").collect::<Vec<&str>>();
                let scale = png_comment.split("scale").collect::<Vec<&str>>();
                let seed = png_comment.split("seed").collect::<Vec<&str>>();
                let sampler = png_comment.split("sampler").collect::<Vec<&str>>();

                file_metadata.prompt = prompt[1].to_string();
                file_metadata.negative_prompt = png_description;
                file_metadata.sampler = sampler[1].to_string();
                file_metadata.seed = seed[1].parse::<u32>().unwrap();
                file_metadata.steps = steps[1].parse::<u32>().unwrap();
                file_metadata.cfg_scale = scale[1].parse::<f32>().unwrap();
                file_metadata.model_hash = png_source;
                file_metadata.vae_hash = None;
            }
            metadata::ArtificialIntelligence::Unknown => {
                eprintln!("Unknown AI");
            }
        }

        Self {
            media_metadata,
            exif_ifd_user_comment: None,
            file_metadata,
        }
    }
}

#[poise::command(slash_command, subcommands("post"), category = "Booru")]
pub async fn aibooru(_: crate::Context<'_>) -> Result<(), crate::Error> {
    Ok(())
}

#[poise::command(slash_command, user_cooldown = 5)]
pub async fn post(
    ctx: crate::Context<'_>,
    #[description = "The ID of the post."] id: i64,
) -> Result<(), crate::Error> {
    let mut _reply = CreateReply {
        content: None,
        ..Default::default()
    };

    // Posts below 0, or at the ID of 0, don't exist on the website.
    let exists = Post::exists(ctx, _reply.clone(), id).await?;
    if exists {
        let posts_show_url = format!("{AIBOORU_URL}/posts/{id}.json");

        let client = reqwest::Client::new();
        let response_text = client.get(&posts_show_url).send().await?.text().await?;
        let response_json = serde_json::from_str(&response_text).unwrap();

        // Posts that don't exist on the website return a JSON object with a "success" key set to false.
        let success = Post::is_success(ctx, &response_json, _reply, id).await?;
        if success {
            let post_data = Post::extract_post_data(&response_json);

            let embed = embed::create_embed_for_booru(
                &post_data,
                AIBOORU_LOGO_PNG,
                id,
                None,
                AIBOORU_URL,
                post_data.file_url.clone(),
                Post::generate_footer(&post_data),
                0x7EB900,
            );

            _reply = CreateReply {
                content: None,
                embeds: vec![embed],
                ..Default::default()
            };

            let media_metadata_png = AiMetadata::extract_media_metadata_png(&response_json);
            let media_metadata_jpeg = AiMetadata::extract_media_metadata_jpeg(&response_json);
            if media_metadata_png.exif_ifd_user_comment.is_some()
                || media_metadata_jpeg.exif_ifd_user_comment.is_some()
            {
                let handle = ctx.send(_reply).await?;

                let mag_right = ReactionType::Unicode("🔎".to_string());

                let message = handle.message().await?;
                message.react(ctx, mag_right).await?;
            }
        }
    }

    Ok(())
}
