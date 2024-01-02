use crate::util::uses::*;

pub fn str_to_json(input: &str) -> HashMap<String, String> {
    let mut json = HashMap::new();

    let lines = input.split('\n');
    for line in lines {
        if let Some(index) = line.find(':') {
            let (key, value) = line.split_at(index);
            let value = value.trim_start_matches(':').trim_start();

            json.insert(key.trim().to_string(), value.to_string());
        }
    }

    json
}
