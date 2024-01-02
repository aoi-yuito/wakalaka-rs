pub mod files;
pub mod settings;
pub mod strings;
pub mod uses;

use crate::util::uses::*;

lazy_static::lazy_static! {
    static ref CACHE_WAKALAKA_DIR: PathBuf = {
        let mut path = dirs::cache_dir().unwrap();
        path.push("wakalaka");
        path
    };
}
