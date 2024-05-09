// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub const AUTHOR_GITHUB_URL: &str = "https://github.com/Kawaxte";
pub const DISCORD_INVITE_URL: &str = "https://discord.gg/jUZVWk7q2q";

pub const STICKER_MAX_SIZE: u32 = 1024 * 512; // 512 KB
pub const STICKER_MAX_DIMENSIONS: (u32, u32) = (320, 320);
pub const STICKER_EXTENSIONS: [&str; 3] = ["apng", "gif", "png"];

pub const EMOJI_EXTENSIONS: [&str; 4] = ["gif", "jpeg", "jpg", "png"];

pub const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CARGO_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const CARGO_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const CARGO_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

lazy_static::lazy_static! {
    pub static ref RES_MASCOT_IMAGE_URL: String = format!(
        "https://raw.githubusercontent.com/{CARGO_AUTHORS}/wakalaka-rs/dev/resources/waka_lichtstern.png"
    );
}
