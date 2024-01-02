use crate::util::uses::*;

#[derive(Default)]
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

impl IntoIterator for FileMetadata {
    type Item = (String, String);
    type IntoIter = std::collections::hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        let mut map = HashMap::new();
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
    pub async fn read(
        mut index: usize,
        attachment: &Attachment,
        map: &mut HashMap<usize, String>,
    ) -> Result<(), Box<dyn error::Error>> {
        let bytes = attachment.download().await.unwrap();

        let mut reader = BufReader::new(&bytes[..]);

        let mut png = Vec::new();
        reader.read_to_end(&mut png)?;

        let mut decoder = Decoder::new(&*png);
        decoder.set_transformations(Transformations::IDENTITY);

        let reader = match decoder.read_info() {
            Ok(reader) => reader,
            Err(err) => {
                println!("Error: {}", err);
                return Ok(());
            }
        };

        let info = reader.info();
        let mut metadata_vec = Vec::new();

        Self::metadata(info, &mut metadata_vec);

        let mut metadata_string = String::new();
        let metadata_keys = vec!["title", "software", "source", "comment"];
        let comment_keys = vec!["prompt", "uc", "sampler", "seed", "steps", "scale"];

        for (key, value) in metadata_vec {
            let key = key.to_lowercase();

            if metadata_keys.contains(&key.as_str()) {
                metadata_string.push_str(&format!("{key}:{value}\n"));
            } else if comment_keys.contains(&key.as_str()) {
                metadata_string.push_str(&format!("{key}:{value}\n"));
            }
        }

        let metadata_json = crate::strings::str_to_json(&metadata_string);

        let file_metadata = Self::file_metadata(metadata_json);

        for (key, value) in file_metadata.into_iter() {
            index += 1;

            map.insert(index, format!("{}: {}", key, value));
        }

        Ok(())
    }

    fn metadata(info: &Info<'_>, metadata_vec: &mut Vec<(String, String)>) {
        if !info.compressed_latin1_text.is_empty() {
            for text in &info.compressed_latin1_text {
                metadata_vec.push((text.keyword.clone(), text.get_text().unwrap()))
            }
        }
        if !info.uncompressed_latin1_text.is_empty() {
            for text in &info.uncompressed_latin1_text {
                metadata_vec.push((text.keyword.clone(), text.text.clone()))
            }
        }
        if !info.utf8_text.is_empty() {
            for text in &info.utf8_text {
                metadata_vec.push((text.keyword.clone(), text.get_text().unwrap()))
            }
        }
    }

    fn file_metadata(metadata_json: HashMap<String, String>) -> FileMetadata {
        let title = metadata_json.get("title").unwrap();
        let software = metadata_json.get("software").unwrap();
        let source = metadata_json.get("source").unwrap();

        let comment = metadata_json.get("comment").unwrap();
        let comment_json =
            serde_json::from_str::<HashMap<String, serde_json::Value>>(comment).unwrap();
        let prompt = comment_json.get("prompt").unwrap();
        let uc = comment_json.get("uc").unwrap();
        let sampler = comment_json.get("sampler").unwrap();
        let seed = comment_json.get("seed").unwrap();
        let steps = comment_json.get("steps").unwrap();
        let cfg_scale = comment_json.get("scale").unwrap();

        let file_metadata = FileMetadata {
            title: title.to_string(),
            software: software.to_string(),
            prompt: prompt.to_string(),
            negative_prompt: uc.to_string(),
            sampler: sampler.to_string(),
            seed: seed.to_string().parse::<i32>().unwrap(),
            steps: steps.to_string().parse::<usize>().unwrap(),
            cfg_scale: cfg_scale.to_string().parse::<f32>().unwrap(),
            model_hash: source.to_string(),
        };
        file_metadata
    }
}

pub fn format_size(size: f64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * KB;

    if size >= MB {
        format!("{:.2} MB", size / MB)
    } else if size >= KB {
        format!("{:.2} KB", size / KB)
    } else {
        format!("{:.0} B", size)
    }
}

pub fn exists(name: &str) -> bool {
    Path::new(name).exists()
}
