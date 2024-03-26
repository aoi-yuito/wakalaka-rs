// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub(crate) mod builders;
pub(crate) mod environment;
pub(crate) mod models;

use regex::Regex;
use tracing::error;

use crate::Throwable;

pub(crate) const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const CARGO_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) const CARGO_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub(crate) const CARGO_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub(crate) const CARGO_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub(crate) const GITHUB_URL: &str = "https://github.com/Kawaxte";

pub(crate) const INVITE_URL: &str = "https://discord.gg/jUZVWk7q2q";
pub(crate) const BOT_INVITE_URL: &str = "https://discord.com/api/oauth2/authorize?client_id=1190718691055251548&permissions=9925535296631&scope=bot";

pub(crate) fn html_to_md(mut input: String) -> Throwable<String> {
    let a_re = Regex::new(r#"<a href="(.*?)">(.*?)</a>"#)?;
    let b_re = Regex::new(r"<b>(.*?)</b>")?;
    let blockquote_re = Regex::new(r"<blockquote>(.*?)</blockquote>")?;
    let br_re = Regex::new(r"<br>")?;
    let code_re = Regex::new(r"<code>(.*?)</code>")?;
    let h1_re = Regex::new(r"<h1>(.*?)</h1>")?;
    let h2_re = Regex::new(r"<h2>(.*?)</h2>")?;
    let h3_re = Regex::new(r"<h3>(.*?)</h3>")?;
    let hr_re = Regex::new(r"<hr>")?;
    let i_re = Regex::new(r"<i>(.*?)</i>")?;
    let u_re = Regex::new(r"<u>(.*?)</u>")?;
    let s_re = Regex::new(r"<s>(.*?)</s>")?;

    input = format!("{}", a_re.replace_all(&input, "[$2]($1)"));
    input = format!("{}", b_re.replace_all(&input, "**$1**"));
    input = format!("{}", blockquote_re.replace_all(&input, "> $1"));
    input = format!("{}", br_re.replace_all(&input, "\n"));
    input = format!("{}", code_re.replace_all(&input, "`$1`"));
    input = format!("{}", hr_re.replace_all(&input, "---"));
    input = format!("{}", h1_re.replace_all(&input, "# $1"));
    input = format!("{}", h2_re.replace_all(&input, "## $1"));
    input = format!("{}", h3_re.replace_all(&input, "### $1"));
    input = format!("{}", i_re.replace_all(&input, "*$1*"));
    input = format!("{}", u_re.replace_all(&input, "__$1__"));
    input = format!("{}", s_re.replace_all(&input, "~~$1~~"));
    Ok(input)
}

pub(crate) fn rgb_to_u32(code: &String) -> Throwable<u32> {
    let mut rgb = code.split(',');

    let r = rgb.next().unwrap_or("0").parse::<u32>()?;
    let g = rgb.next().unwrap_or("0").parse::<u32>()?;
    let b = rgb.next().unwrap_or("0").parse::<u32>()?;

    let hex = format!("{r:02X}{g:02X}{b:02X}");
    Ok(hex_to_u32(&hex))
}

pub(crate) fn hex_to_u32(code: &String) -> u32 {
    let hex_code: String = code.chars().filter(|c| c.is_digit(16)).collect();

    match u32::from_str_radix(&hex_code, 16) {
        Ok(colour) => colour,
        Err(why) => {
            error!("Failed to parse {code:?}: {why:?}");
            0
        }
    }
}
