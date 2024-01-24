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
    all::{colours::branding, Guild, ShardId, User},
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
    gateway::ConnectionStage,
    model::Timestamp,
};
use std::fmt::Write;
use tokio::time::Duration;

pub fn warnings_command_embed(
    user: &User,
    uuids: Vec<String>,
    moderator_ids: Vec<i64>,
    reasons: Vec<String>,
) -> CreateEmbed {
    //  |(PFP) {user_name}                |
    //  | ID      | Moderator | Reason    |
    //  |---------|-----------|-----------|
    //  | <uuid1> | <@{id1}>  | {reason1} |
    //  | <uuid2> | <@{id2}>  | {reason2} |
    //  | <uuid3> | <@{id3}>  | {reason3} |

    let (user_name, user_avatar_url) = (
        &user.name,
        user.avatar_url().unwrap_or(user.default_avatar_url()),
    );

    let embed_author = CreateEmbedAuthor::new(user_name).icon_url(user_avatar_url);

    let mut id_field = String::new();
    let mut moderator_field = String::new();
    let mut reason_field = String::new();
    for ((uuid, moderator_id), reason) in uuids.iter().zip(moderator_ids.iter()).zip(reasons.iter())
    {
        writeln!(id_field, "{uuid}").unwrap();
        writeln!(moderator_field, "<@{moderator_id}>").unwrap();
        writeln!(reason_field, "{reason}").unwrap();
    }

    let mut embed_fields = Vec::new();
    embed_fields.push(("ID", id_field, true));
    embed_fields.push(("Moderator", moderator_field, true));
    embed_fields.push(("Reason", reason_field, true));

    CreateEmbed::default()
        .author(embed_author)
        .fields(embed_fields)
}

pub fn suggest_command_embed(
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

pub fn colour_command_embed(colour: u32, url: &String, json: &serde_json::Value) -> CreateEmbed {
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

pub fn avatar_command_embed(name: &String, avatar_url: String) -> CreateEmbed {
    let embed_author = CreateEmbedAuthor::new(name).icon_url(avatar_url.clone());

    CreateEmbed::default()
        .author(embed_author)
        .image(avatar_url)
}

pub fn banned_command_embed(name: &String, avatar_url: String, banner_url: String) -> CreateEmbed {
    let embed_author = CreateEmbedAuthor::new(name).icon_url(avatar_url.clone());

    CreateEmbed::default()
        .author(embed_author)
        .image(banner_url)
}

pub fn roles_command_embed(guild: &Guild, fields: Vec<(&str, String, bool)>) -> CreateEmbed {
    let guild_name = &guild.name;
    let guild_icon_url = guild.icon_url().unwrap_or_default();

    let embed_author = CreateEmbedAuthor::new(guild_name).icon_url(guild_icon_url);

    CreateEmbed::default()
        .title("Roles")
        .author(embed_author)
        .fields(fields)
}

pub fn ping_command_embed(
    elapsed_time: Duration,
    ids: Vec<&ShardId>,
    stages: Vec<ConnectionStage>,
    latencies: Vec<Option<Duration>>,
) -> CreateEmbed {
    //  | Pong!                             |
    //  | Shard    | State      | Latency   |
    //  |----------|------------|-----------|
    //  | <id1>    | <@{state1> | {ms1}     |
    //  | <id2>    | <@{state2> | {ms2}     |
    //  | Response |                        |
    //  |----------|                        |
    //  | {response_latency}                |

    let mut id_field = String::new();
    let mut stage_field = String::new();
    let mut latency_field = String::new();

    for ((id, stage), latency) in ids.iter().zip(stages.iter()).zip(latencies.iter()) {
        writeln!(id_field, "{id}").unwrap();
        writeln!(stage_field, "{stage}").unwrap();

        if latency.is_some() {
            let latency = latency.unwrap_or_default();

            writeln!(latency_field, "{latency:.2?}").unwrap();
        } else {
            writeln!(latency_field, "N/A").unwrap();
        }
    }

    let embed_fields = vec![
        ("Shard", id_field, true),
        ("State", stage_field, true),
        ("Latency", latency_field, true),
    ];
    let embed_footer = CreateEmbedFooter::new(format!("{elapsed_time:.2?}"));

    CreateEmbed::default()
        .title("Pong!")
        .fields(embed_fields)
        .footer(embed_footer)
}

pub fn info_command_embed(icon_url: &String, constants: [&str; 6]) -> CreateEmbed {
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

pub fn error_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("❌ {message}"))
        .colour(branding::RED)
}

pub fn warn_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("⚠️ {message}"))
        .colour(branding::YELLOW)
}

pub fn ok_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("✅ {message}"))
        .colour(branding::GREEN)
}

pub fn info_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!(":information_source: {message}"))
        .colour(branding::BLURPLE)
}

pub fn message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default().description(format!("{message}"))
}
