// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use regex::Regex;
use wakalaka_core::types::Throwable;

pub fn html_to_discord_md(mut input: String) -> Throwable<String> {
    let replacements = [
        (Regex::new(r#"<a href="(.*?)">(.*?)</a>"#)?, "[$2]($1)"),
        (Regex::new(r"<b>(.*?)</b>")?, "**$1**"),
        (Regex::new(r"<blockquote>(.*?)</blockquote>")?, "> $1"),
        (Regex::new(r"<br>")?, "\n"),
        (Regex::new(r"<code>(.*?)</code>")?, "`$1`"),
        (Regex::new(r"<dd>(.*?)</dd>")?, "$1"),
        (Regex::new(r"<dt>(.*?)</dt>")?, "\t$1"),
        (Regex::new(r"<hr>")?, "---"),
        (Regex::new(r"<h1>(.*?)</h1>")?, "# $1"),
        (Regex::new(r"<h2>(.*?)</h2>")?, "## $1"),
        (Regex::new(r"<h3>(.*?)</h3>")?, "### $1"),
        (Regex::new(r"<i>(.*?)</i>")?, "*$1*"),
        (Regex::new(r"<li>(.*?)</li>")?, "- $1"),
        (Regex::new(r"<ol>(.*?)</ol>")?, "1. $1"),
        (Regex::new(r"<u>(.*?)</u>")?, "__$1__"),
        (Regex::new(r"<s>(.*?)</s>")?, "~~$1~~"),
    ];

    for (re, replacement) in replacements {
        input = format!("{}", re.replace_all(&input, replacement));
    }

    Ok(input)
}
