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

pub struct FileMetadata {
    title: String,
    software: String,
    prompt: String,
    negative_prompt: String,
    sampler: String,
    seed: i32,
    steps: usize,
    cfg_scale: f32,
    model_hash: String,
}

impl Default for FileMetadata {
    fn default() -> Self {
        Self {
            title: String::from("Unknown"),
            software: String::from("Unknown"),
            prompt: String::from("Unknown"),
            negative_prompt: String::from("Unknown"),
            sampler: String::from("Unknown"),
            seed: -1,
            steps: 0,
            cfg_scale: -1.0,
            model_hash: String::from("Unknown"),
        }
    }
}

impl IntoIterator for FileMetadata {
    type Item = (String, String);
    type IntoIter = linked_hash_map::IntoIter<String, String>;

    fn into_iter(self) -> linked_hash_map::IntoIter<String, String> {
        let mut map = LinkedHashMap::new();
        map.insert("title".to_string(), self.title);
        map.insert("software".to_string(), self.software);
        map.insert("prompt".to_string(), self.prompt);
        map.insert("negative_prompt".to_string(), self.negative_prompt);
        map.insert("sampler".to_string(), self.sampler);
        map.insert("seed".to_string(), self.seed.to_string());
        map.insert("steps".to_string(), self.steps.to_string());
        map.insert("cfg_scale".to_string(), self.cfg_scale.to_string());
        map.insert("model_hash".to_string(), self.model_hash);
        map.into_iter()
    }
}

impl FileMetadata {
    fn new(json: LinkedHashMap<String, String>) -> Self {
        let title = json.get("title").unwrap();
        let software = json.get("software").unwrap();
        let source = json.get("source").unwrap();

        let comment = json.get("comment").unwrap();
        let comment_json =
            serde_json::from_str::<LinkedHashMap<String, serde_json::Value>>(comment).unwrap();
        let prompt = comment_json.get("prompt").unwrap();
        let uc = comment_json.get("uc").unwrap();
        let sampler = comment_json.get("sampler").unwrap();
        let seed = comment_json.get("seed").unwrap();
        let steps = comment_json.get("steps").unwrap();
        let cfg_scale = comment_json.get("scale").unwrap();

        Self {
            title: title.to_string(),
            software: software.to_string(),
            prompt: prompt.to_string(),
            negative_prompt: uc.to_string(),
            sampler: sampler.to_string(),
            seed: seed.to_string().parse::<i32>().unwrap(),
            steps: steps.to_string().parse::<usize>().unwrap(),
            cfg_scale: cfg_scale.to_string().parse::<f32>().unwrap(),
            model_hash: source.to_string(),
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

            let key = format!("**{}**", value.split(":").collect::<Vec<&str>>()[0]);
            let mut new_value = value.split(":").collect::<Vec<&str>>()[1].to_string();
            new_value = new_value.replace("\\n", "\n").replace("\"", "");

            if key == "**title**" {
                *embed = cloned_embed.clone().description(new_value);
                continue;
            }

            let metadata = cloned_embed
                .clone()
                .field(key.to_uppercase(), new_value, false);
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
        Err(err) => {
            println!("Error: {}", err);
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

    let metadata_keys = vec!["title", "software", "source", "comment"];
    let comment_keys = vec!["prompt", "uc", "sampler", "seed", "steps", "scale"];

    for (key, value) in vector {
        let key = key.to_lowercase();

        if metadata_keys.contains(&key.as_str()) {
            metadata_string.push_str(&format!("{key}:{value}\n"));
        } else if comment_keys.contains(&key.as_str()) {
            metadata_string.push_str(&format!("{key}:{value}\n"));
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
