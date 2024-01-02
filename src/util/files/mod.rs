pub mod metadata;

use crate::util::uses::*;

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
