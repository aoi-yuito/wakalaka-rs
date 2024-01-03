pub mod metadata;

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

pub fn is_attachment_jpeg(attachment: &Attachment) -> bool {
    attachment.filename.ends_with(".jpg")
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
