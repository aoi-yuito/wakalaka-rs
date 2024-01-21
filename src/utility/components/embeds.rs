// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

use chrono::{NaiveDateTime, TimeZone, Utc};
use serenity::{
    all::{colours::branding, ShardId, User},
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
    gateway::ConnectionStage,
    model::Timestamp,
};
use std::fmt::Write;
use tokio::time::Duration;

pub(crate) fn warnings_embed(
    case_ids: Vec<i32>,
    user: &User,
    user_name: &String,
    moderator_ids: Vec<i64>,
    reasons: Vec<String>,
) -> CreateEmbed {
    //  |(PFP) {user_name}             |
    //  | Case | Moderator | Reason    |
    //  |------|-----------|-----------|
    //  | 1    | <@{id1}>  | {reason1} |
    //  | 2    | <@{id2}>  | {reason2} |
    //  | 3    | <@{id3}>  | {reason3} |

    let user_icon_url = user.avatar_url().unwrap_or(user.default_avatar_url());

    let embed_author = CreateEmbedAuthor::new(user_name).icon_url(user_icon_url);

    let mut case_field = String::new();
    let mut moderator_field = String::new();
    let mut reason_field = String::new();
    for ((case_id, moderator_id), reason) in case_ids
        .iter()
        .zip(moderator_ids.iter())
        .zip(reasons.iter())
    {
        writeln!(case_field, "{case_id}").unwrap();
        writeln!(moderator_field, "<@{moderator_id}>").unwrap();
        writeln!(reason_field, "{reason}").unwrap();
    }

    let mut embed_fields = Vec::new();
    embed_fields.push(("Case", case_field, true));
    embed_fields.push(("Moderator", moderator_field, true));
    embed_fields.push(("Reason", reason_field, true));

    CreateEmbed::default()
        .author(embed_author)
        .fields(embed_fields)
}

pub(crate) fn suggest_embed(
    name: &String,
    avatar_url: String,
    description: &String,
    created_at: NaiveDateTime,
) -> CreateEmbed {
    let embed_author = CreateEmbedAuthor::new(name).icon_url(avatar_url);

    let now = Timestamp::from(Utc.from_utc_datetime(&created_at));

    CreateEmbed::default()
        .author(embed_author)
        .description(description)
        .timestamp(Timestamp::from(now))
}

pub(crate) fn colour_embed(colour: u32, url: &String, json: &serde_json::Value) -> CreateEmbed {
    let name = json["name"]["value"].as_str().unwrap();
    let hex = format!("{:06X}", colour);
    let rgb = json["rgb"]["value"].as_str().unwrap();
    let hsl = json["hsl"]["value"].as_str().unwrap();

    let rgb_stripped = rgb
        .strip_prefix("rgb(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .to_string();
    let hsl_stripped = hsl
        .strip_prefix("hsl(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .to_string();

    let mut embed_fields = Vec::new();
    embed_fields.push(("Hexadecimal", hex, true));
    embed_fields.push(("RGB", rgb_stripped, true));
    embed_fields.push(("HSL", hsl_stripped, true));

    CreateEmbed::default()
        .title(name)
        .fields(embed_fields)
        .image(url)
        .colour(colour)
}

pub(crate) fn avatar_embed(name: &String, avatar_url: String) -> CreateEmbed {
    let embed_author = CreateEmbedAuthor::new(name).icon_url(avatar_url.clone());

    CreateEmbed::default()
        .author(embed_author)
        .image(avatar_url)
}

pub(crate) fn banner_embed(name: &String, avatar_url: String, banner_url: String) -> CreateEmbed {
    let embed_author = CreateEmbedAuthor::new(name).icon_url(avatar_url.clone());

    CreateEmbed::default()
        .author(embed_author)
        .image(banner_url)
}

pub(crate) fn ping_embed(
    elapsed_time: Duration,
    shard_id: &ShardId,
    stage: ConnectionStage,
    latency: Option<Duration>,
) -> CreateEmbed {
    if latency.is_some() {
        // If this doesn't get the "Some(value)" formatting fuck out of here, shit the bed with a default, fresh out from under my foreskin.
        let latency = latency.unwrap_or_default();

        CreateEmbed::default()
            .title("Pong!")
            .field(
                "Shards",
                format!("{shard_id} ({stage}, {latency:.2?})"),
                true,
            )
            .field("Response", format!("{elapsed_time:.2?}"), true)
    } else {
        CreateEmbed::default()
            .title("Pong!")
            .field("Shards", format!("{shard_id} ({stage})"), true)
            .field("Response", format!("{elapsed_time:.2?}"), true)
    }
}

pub(crate) fn info_embed(icon_url: &String, constants: [&str; 6]) -> CreateEmbed {
    let author = match constants[2].split(',').next() {
        Some(value) => value,
        None => "No author found",
    };
    let embed_author = CreateEmbedAuthor::new(author).icon_url(icon_url);

    let footer = format!("Powered by Rust {}", constants[5]);
    let embed_footer = CreateEmbedFooter::new(footer);

    CreateEmbed::default()
        .author(embed_author)
        .title(format!("{} v{}", constants[0], constants[1]))
        .description(constants[3])
        .url(format!("{}/{}", constants[4], constants[0]))
        .footer(embed_footer)
}

pub(crate) fn error_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("❌ {message}"))
        .colour(branding::RED)
}

pub(crate) fn warning_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("⚠️ {message}"))
        .colour(branding::YELLOW)
}

pub(crate) fn ok_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("✅ {message}"))
        .colour(branding::GREEN)
}

pub(crate) fn message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("{message}"))
        .colour(branding::BLURPLE)
}
