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

pub async fn message(msg: &Message) -> Result<(), crate::Error> {
    let attachments = &msg.attachments;
    if !attachments.is_empty() {
        let mut metadata = HashMap::new();

        for (index, attachment) in attachments.iter().enumerate() {
            FileMetadata::read(index, attachment, &mut metadata)
                .await
                .expect("Failed to read metadata");
        }
    }

    Ok(())
}
