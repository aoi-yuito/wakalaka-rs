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
    all::{
        colours::{branding, css},
        Guild, ShardId, User, UserId,
    },
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
    gateway::ConnectionStage,
    model::{Colour, Timestamp},
};
use std::fmt::Write;
use tokio::time::Duration;

pub fn warnings_command_embed(
    user: &User,
    uuids: Vec<&String>,
    moderator_ids: Vec<&i64>,
    reasons: Vec<&String>,
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
    let embed_author = CreateEmbedAuthor::new(name).icon_url(&avatar_url);

    CreateEmbed::default()
        .author(embed_author)
        .image(avatar_url)
        .colour(branding::BLURPLE)
}

pub fn banner_command_embed(name: &String, avatar_url: String, banner_url: String) -> CreateEmbed {
    let embed_author = CreateEmbedAuthor::new(name).icon_url(&avatar_url);

    CreateEmbed::default()
        .author(embed_author)
        .image(banner_url)
        .colour(branding::BLURPLE)
}

pub fn roles_command_embed(guild: &Guild, fields: Vec<(&str, String, bool)>) -> CreateEmbed {
    let guild_name = &guild.name;
    let guild_icon_url = guild.icon_url().unwrap_or_default();

    let embed_author = CreateEmbedAuthor::new(guild_name).icon_url(guild_icon_url);

    CreateEmbed::default()
        .title("Roles")
        .author(embed_author)
        .fields(fields)
        .colour(branding::BLURPLE)
}

pub fn ping_command_embed(
    elapsed_time: Duration,
    ids: Vec<&ShardId>,
    stages: Vec<ConnectionStage>,
    latencies: Vec<Option<Duration>>,
    memory: (f64, f64),
) -> CreateEmbed {
    //  | Pong!                                       |
    //  | Shard    | State           | Latency        |
    //  |----------|-----------------|----------------|
    //  | <id1>    | <@{state1>      | {ms1}          |
    //  | <id2>    | <@{state2>      | {ms2}          |
    //  | {response_latency} | {used} MB / {total} MB |

    let mut id_field = String::new();
    let mut stage_field = String::new();
    let mut latency_field = String::new();

    for ((id, stage), latency) in ids.iter().zip(stages.iter()).zip(latencies.iter()) {
        writeln!(id_field, "{id}").expect("Couldn't write to 'id_field'");
        writeln!(stage_field, "{stage}").expect("Couldn't write to 'stage_field'");

        if latency.is_some() {
            let latency = latency.unwrap_or_default();

            writeln!(latency_field, "{latency:.2?}").expect("Couldn't write to 'latency_field'");
        } else {
            writeln!(latency_field, "N/A").expect("Couldn't write to 'latency_field'");
        }
    }

    let embed_fields = vec![
        ("💎 Shard", id_field, true),
        ("📶 State", stage_field, true),
        ("🕓 Latency", latency_field, true),
    ];
    let embed_footer = CreateEmbedFooter::new(format!(
        "🕓{elapsed_time:.2?} - 🖥️{:.2} MB / {:.2} MB",
        memory.0, memory.1
    ));

    CreateEmbed::default()
        .title("Pong!")
        .fields(embed_fields)
        .footer(embed_footer)
        .colour(branding::BLURPLE)
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
        .colour(branding::BLURPLE)
}

/// If you're ever going to add an Economy system or something, you should modify this embed to contain this kind of information.
pub fn lookup_user_command_embed(
    id: &UserId,
    name: &String,
    icon_url: &String,
    accent_colour: &Option<Colour>,
) -> CreateEmbed {
    let (embed_author, embed_colour, embed_footer) = (
        CreateEmbedAuthor::new(name).icon_url(icon_url),
        match accent_colour {
            Some(colour) => *colour,
            None => branding::BLURPLE,
        },
        CreateEmbedFooter::new(id.to_string()),
    );

    CreateEmbed::default()
        .author(embed_author)
        .colour(embed_colour)
        .footer(embed_footer)
}

pub fn lookup_server_command_embed(guild: &Guild, owner: &User) -> CreateEmbed {
    // |(SPFP) {server_name}              |
    // | Roles          | Emojis          |
    // |----------------| ----------------|
    // | {role_count}   | {emoji_count}   |
    // |                                  |
    // | Members        | Channels        |
    // |----------------| ----------------|
    // | {member_count} | {channel_count} |

    let guild_id = guild.id;
    let guild_created_at = guild_id.created_at();

    let (
        guild_name,
        guild_icon_url,
        guild_banner_url,
        guild_description,
        guild_role_count,
        guild_emoji_count,
        guild_member_count,
        guild_channel_count,
    ) = (
        &guild.name,
        guild.icon_url().unwrap_or_default(),
        guild.banner_url().unwrap_or_default(),
        &guild.description,
        guild.roles.len(),
        guild.emojis.len(),
        guild.member_count,
        guild.channels.len(),
    );

    let (owner_name, owner_avatar_url) = (
        &owner.name,
        owner.avatar_url().unwrap_or(owner.default_avatar_url()),
    );

    let embed_author = CreateEmbedAuthor::new(guild_name).icon_url(guild_icon_url);

    let mut guild_roles_field = String::new();
    let mut guild_emojis_field = String::new();
    let mut guild_members_field = String::new();
    let mut guild_channels_field = String::new();
    writeln!(guild_roles_field, "{guild_role_count}").unwrap();
    writeln!(guild_emojis_field, "{guild_emoji_count}").unwrap();
    writeln!(guild_members_field, "{guild_member_count}").unwrap();
    writeln!(guild_channels_field, "{guild_channel_count}").unwrap();

    let embed_fields = vec![
        ("Roles", guild_roles_field, true),
        ("Emojis", guild_emojis_field, true),
        ("Members", guild_members_field, true),
        ("Channels", guild_channels_field, true),
    ];

    let embed_footer = CreateEmbedFooter::new(owner_name).icon_url(owner_avatar_url);

    if let Some(guild_description) = guild_description {
        CreateEmbed::default()
            .author(embed_author)
            .description(guild_description)
            .fields(embed_fields)
            .image(guild_banner_url)
            .footer(embed_footer)
            .timestamp(guild_created_at)
            .colour(branding::BLURPLE)
    } else {
        CreateEmbed::default()
            .author(embed_author)
            .fields(embed_fields)
            .image(guild_banner_url)
            .footer(embed_footer)
            .timestamp(guild_created_at)
            .colour(branding::BLURPLE)
    }
}

pub fn error_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("{message}"))
        .colour(css::DANGER)
}

pub fn warn_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("{message}"))
        .colour(css::WARNING)
}

pub fn ok_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("{message}"))
        .colour(css::POSITIVE)
}

pub fn info_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(format!("{message}"))
        .colour(Colour::BLUE)
}

pub fn message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default().description(format!("{message}"))
}
