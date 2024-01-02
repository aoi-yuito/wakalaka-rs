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

pub fn str_to_json(input: &str) -> LinkedHashMap<String, String> {
    let mut json = LinkedHashMap::new();

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
