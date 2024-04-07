// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use regex::Regex;
use wakalaka_core::types::Throwable;

pub fn html_to_discord_md(mut input: String) -> Throwable<String> {
    let a_href_re = Regex::new(r#"<a href="(.*?)">(.*?)</a>"#)?;
    let b_re = Regex::new(r"<b>(.*?)</b>")?;
    let blockquote_re = Regex::new(r"<blockquote>(.*?)</blockquote>")?;
    let br_re = Regex::new(r"<br>")?;
    let code_re = Regex::new(r"<code>(.*?)</code>")?;
    let dd_re = Regex::new(r"<dd>(.*?)</dd>")?;
    let dt_re = Regex::new(r"<dt>(.*?)</dt>")?;
    let h1_re = Regex::new(r"<h1>(.*?)</h1>")?;
    let h2_re = Regex::new(r"<h2>(.*?)</h2>")?;
    let h3_re = Regex::new(r"<h3>(.*?)</h3>")?;
    let hr_re = Regex::new(r"<hr>")?;
    let i_re = Regex::new(r"<i>(.*?)</i>")?;
    let li = Regex::new(r"<li>(.*?)</li>")?;
    let ol = Regex::new(r"<ol>(.*?)</ol>")?;
    let u_re = Regex::new(r"<u>(.*?)</u>")?;
    let s_re = Regex::new(r"<s>(.*?)</s>")?;

    input = format!("{}", a_href_re.replace_all(&input, "[$2]($1)"));
    input = format!("{}", b_re.replace_all(&input, "**$1**"));
    input = format!("{}", blockquote_re.replace_all(&input, "> $1"));
    input = format!("{}", br_re.replace_all(&input, "\n"));
    input = format!("{}", code_re.replace_all(&input, "`$1`"));
    input = format!("{}", dd_re.replace_all(&input, "$1"));
    input = format!("{}", dt_re.replace_all(&input, "\t$1"));
    input = format!("{}", hr_re.replace_all(&input, "---"));
    input = format!("{}", h1_re.replace_all(&input, "# $1"));
    input = format!("{}", h2_re.replace_all(&input, "## $1"));
    input = format!("{}", h3_re.replace_all(&input, "### $1"));
    input = format!("{}", i_re.replace_all(&input, "*$1*"));
    input = format!("{}", li.replace_all(&input, "- $1"));
    input = format!("{}", ol.replace_all(&input, "1. $1"));
    input = format!("{}", u_re.replace_all(&input, "__$1__"));
    input = format!("{}", s_re.replace_all(&input, "~~$1~~"));
    Ok(input)
}
