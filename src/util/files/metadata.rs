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
use crate::util::uses::*;

enum Software {
    A1111,
    NovelAI,
    Unknown,
}

impl Software {
    fn new(json: &LinkedHashMap<String, String>) -> Self {
        if json.contains_key("parameters") {
            return Self::A1111;
        } else if json.contains_key("comment") {
            return Self::NovelAI;
        } else {
            return Self::Unknown;
        }
    }
}

pub struct FileMetadata {
    prompt: String,
    negative_prompt: String,
    sampler: String,
    seed: u32,
    steps: usize,
    cfg_scale: f32,
    model_hash: String,
    vae_hash: Option<String>, // A1111
}

impl Default for FileMetadata {
    fn default() -> Self {
        Self {
            prompt: "Unknown".to_string(),
            negative_prompt: "Unknown".to_string(),
            sampler: "Unknown".to_string(),
            seed: u32::MIN,
            steps: usize::MIN,
            cfg_scale: f32::MIN,
            model_hash: "Unknown".to_string(),
            vae_hash: Some("Unknown".to_string()),
        }
    }
}

impl IntoIterator for FileMetadata {
    type Item = (String, String);
    type IntoIter = linked_hash_map::IntoIter<String, String>;

    fn into_iter(self) -> linked_hash_map::IntoIter<String, String> {
        let mut map = LinkedHashMap::new();
        map.insert("prompt".to_string(), self.prompt);
        map.insert("negative_prompt".to_string(), self.negative_prompt);
        map.insert("sampler".to_string(), self.sampler);
        map.insert("seed".to_string(), self.seed.to_string());
        map.insert("steps".to_string(), self.steps.to_string());
        map.insert("cfg_scale".to_string(), self.cfg_scale.to_string());
        map.insert("model_hash".to_string(), self.model_hash);
        if self.vae_hash.is_some() {
            map.insert("vae_hash".to_string(), self.vae_hash.unwrap());
        }

        map.into_iter()
    }
}

impl FileMetadata {
    fn new(json: LinkedHashMap<String, String>) -> Self {
        let software = Software::new(&json);
        match software {
            Software::A1111 => Self::new_a1111(json),
            Software::NovelAI => Self::new_novelai(json),
            Software::Unknown => Self::default(),
        }
    }

    fn new_a1111(json: LinkedHashMap<String, String>) -> Self {
        let prompt = json.get("parameters").unwrap();
        let negative_prompt = json.get("Negative prompt").unwrap();
        let steps_ = json.get("Steps").unwrap();
        let split_steps_ = steps_.split(",").collect::<Vec<&str>>();

        let steps = split_steps_[0].parse::<usize>().unwrap();
        let sampler = split_steps_[1].split(":").collect::<Vec<&str>>()[1].trim();
        let seed = split_steps_[3].split(":").collect::<Vec<&str>>()[1].trim();
        let cfg_scale = split_steps_[2].split(":").collect::<Vec<&str>>()[1].trim();
        let model = split_steps_[6].split(":").collect::<Vec<&str>>()[1].trim();
        let model_hash = split_steps_[5].split(":").collect::<Vec<&str>>()[1]
            .trim()
            .to_uppercase();
        let vae = split_steps_[8].split(":").collect::<Vec<&str>>()[1].trim();
        let vae_hash = split_steps_[7].split(":").collect::<Vec<&str>>()[1]
            .trim()
            .to_uppercase();

        Self {
            prompt: prompt.to_string(),
            negative_prompt: negative_prompt.to_string(),
            sampler: sampler.to_string(),
            seed: seed.parse::<u32>().unwrap(),
            steps,
            cfg_scale: cfg_scale.parse::<f32>().unwrap(),
            model_hash: format!("{model} {model_hash}"),
            vae_hash: Some(format!("{vae} {vae_hash}")),
        }
    }

    fn new_novelai(json: LinkedHashMap<String, String>) -> Self {
        let source = json.get("source").unwrap();
        let comment = json.get("comment").unwrap();
        let comment_json =
            serde_json::from_str::<LinkedHashMap<String, serde_json::Value>>(comment).unwrap();

        let prompt = comment_json.get("prompt").unwrap();
        let uc = comment_json.get("uc").unwrap();
        let sampler = comment_json.get("sampler").unwrap();
        let seed = comment_json.get("seed").unwrap().to_string();
        let steps = comment_json.get("steps").unwrap().to_string();
        let cfg_scale = comment_json.get("scale").unwrap().to_string();

        Self {
            prompt: prompt.to_string(),
            negative_prompt: uc.to_string(),
            sampler: sampler.to_string(),
            seed: seed.parse::<u32>().unwrap(),
            steps: steps.parse::<usize>().unwrap(),
            cfg_scale: cfg_scale.parse::<f32>().unwrap(),
            model_hash: source.to_string(),
            vae_hash: None,
        }
    }

    pub async fn attachment_metadata(ctx: &Context, http: Arc<Http>, msg: &Message) {
        let mut metadata = LinkedHashMap::new();
        let mut metadata_read_res_ok = false;

        let mut message = CreateMessage::default();

        let settings = Settings::read_settings().unwrap();

        let channel_id = msg.channel_id;
        if channel_id != ChannelId::from(settings.metadata_channel_id) {
            return;
        }
        let message_id = msg.id;

        let attachments = &msg.attachments;
        if event::message::is_attachment(msg) {
            for (index, attachment) in attachments.iter().enumerate() {
                let metadata_read_res =
                    Self::read_attachment_metadata(index, attachment, &mut metadata).await;

                metadata_read_res_ok = metadata_read_res.is_ok();
                if metadata_read_res_ok {
                    for attachment in attachments {
                        let mut file_name = attachment.filename.as_str();
                        file_name = file_name.split(".").collect::<Vec<&str>>()[0];

                        let mut embed =
                            embed::create_embed_for_metadata(file_name.to_string(), 0x5D67F6);

                        Self::format_attachment_metadata(&metadata, &mut embed);

                        message = message.embed(embed);
                    }
                }
            }
        }

        if metadata_read_res_ok {
            let mag_right = ReactionType::Unicode("🔎".to_string());

            event::reaction_add::add_reaction_to_message(
                http.clone(),
                channel_id,
                message_id,
                mag_right.clone(),
            )
            .await;

            event::message::send_dm_if_embed_attachment_reactive(
                msg, ctx, mag_right, channel_id, http, message,
            )
            .await;
        }
    }

    async fn read_attachment_metadata(
        mut index: usize,
        attachment: &Attachment,
        map: &mut LinkedHashMap<usize, String>,
    ) -> Result<(), Box<dyn error::Error>> {
        let bytes = attachment.download().await?;

        let png = decode_png(bytes)?;

        let metadata_string = metadata_string(png);
        let metadata_json = strings::str_to_json(&metadata_string);

        let file_metadata = Self::new(metadata_json);
        insert_into_map(&mut index, file_metadata, map);

        Ok(())
    }

    fn format_attachment_metadata(map: &LinkedHashMap<usize, String>, embed: &mut CreateEmbed) {
        for (_, value) in map.iter() {
            let cloned_embed = embed.clone();

            let parts: Vec<&str> = value.splitn(2, ':').collect();
            let key_input = format!("**{}**", parts[0]);
            let key = strings::snakecase_to_titlecase(&key_input);

            let mut new_value = parts[1].to_string();
            new_value = new_value.replace("\\n", "\n").replace("\"", "");

            let metadata = cloned_embed
                .clone()
                .field(key.to_ascii_uppercase(), new_value, false);
            *embed = metadata;
        }
    }
}

fn decode_png(bytes: Vec<u8>) -> Result<Vec<(String, String)>, Box<dyn error::Error>> {
    let mut reader = BufReader::new(&bytes[..]);

    let mut png = Vec::new();
    reader.read_to_end(&mut png)?;

    let mut decoder = Decoder::new(&*png);
    decoder.set_transformations(Transformations::IDENTITY);

    let reader = match decoder.read_info() {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Error: {e}");
            return Ok(Vec::new());
        }
    };

    let png_info = reader.info();
    let png_metadata = metadata_text(png_info);
    Ok(png_metadata)
}

fn metadata_text(info: &Info<'_>) -> Vec<(String, String)> {
    let mut metadata_text = Vec::new();

    if !info.compressed_latin1_text.is_empty() {
        for text in &info.compressed_latin1_text {
            metadata_text.push((text.keyword.clone(), text.get_text().unwrap()))
        }
    }
    if !info.uncompressed_latin1_text.is_empty() {
        for text in &info.uncompressed_latin1_text {
            metadata_text.push((text.keyword.clone(), text.text.clone()))
        }
    }
    if !info.utf8_text.is_empty() {
        for text in &info.utf8_text {
            metadata_text.push((text.keyword.clone(), text.get_text().unwrap()))
        }
    }

    metadata_text
}

fn metadata_string(vector: Vec<(String, String)>) -> String {
    let mut metadata_string = String::new();

    let a1111 = vec!["parameters", "Negative prompt", "Steps"];

    let novelai = vec!["source", "comment"];
    let novelai_comment = vec!["prompt", "uc", "sampler", "seed", "steps", "scale"];

    for (key, value) in vector {
        let key = key.to_lowercase();

        if novelai.contains(&key.as_str()) {
            metadata_string.push_str(&format!("{}: {}\n", key, value));
        } else if novelai_comment.contains(&key.as_str()) {
            metadata_string.push_str(&format!("{}: {}\n", key, value));
        } else if a1111.contains(&key.as_str()) {
            metadata_string.push_str(&format!("{}: {}\n", key, value));
        }
    }

    metadata_string
}

fn insert_into_map(
    index: &mut usize,
    file_metadata: FileMetadata,
    map: &mut LinkedHashMap<usize, String>,
) {
    for (key, value) in file_metadata.into_iter() {
        *index += 1;

        map.insert(*index, format!("{}: {}", key, value));
    }
}
