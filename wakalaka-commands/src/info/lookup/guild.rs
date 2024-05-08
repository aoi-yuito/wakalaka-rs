// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateEmbedAuthor, CreateEmbedFooter, Guild};

use wakalaka_core::{
    accessors, builders,
    types::{Context, Throwable},
};

#[poise::command(
    slash_command,
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Query a server yours truly is in.
pub(super) async fn guild(
    ctx: Context<'_>,
    #[description = "Server to query, if any."] guild: Option<Guild>,
) -> Throwable<()> {
    let guild = guild.unwrap_or(accessors::guilds::fetch_guild(ctx)?);
    let guild_id = guild.id;
    let guild_name = &guild.name;
    let guild_description = guild.description.as_deref().unwrap_or_default();
    let guild_icon_url = guild.icon_url().unwrap_or_default();
    let guild_banner_url = guild.banner_url().unwrap_or_default();
    let guild_owner_id = guild.owner_id;
    let guild_owner = guild_owner_id.to_user(ctx).await?;
    let guild_owner_name = &guild_owner.name;
    let guild_owner_face = guild_owner.face();
    let guild_roles = guild.roles;
    let guild_emojis = guild.emojis;
    let guild_stickers = guild.stickers;
    let guild_members = guild
        .members
        .iter()
        .filter(|member| !member.1.user.bot && !member.1.user.system)
        .collect::<Vec<_>>(); // Bots and system users are not a metric for population.
    let guild_channels = guild.channels;
    let guild_created_at = guild_id.created_at();

    let guild_role_count = guild_roles.len();
    let guild_emoji_count = guild_emojis.len();
    let guild_sticker_count = guild_stickers.len();
    let guild_member_count = guild_members.len();
    let guild_channel_count = guild_channels.len();

    let embed_author = CreateEmbedAuthor::new(guild_owner_name).icon_url(guild_owner_face);
    let embed_fields = vec![
        ("🛡️ Roles", format!("{guild_role_count}"), true),
        ("😃 Emojis", format!("{guild_emoji_count}"), true),
        ("🖼️ Stickers", format!("{guild_sticker_count}"), true),
        ("👥 Members", format!("{guild_member_count}"), true),
        ("🌐 Channels", format!("{guild_channel_count}"), true),
    ];
    let embed_footer = CreateEmbedFooter::new(format!("🆔{guild_id}"));
    let embed = builders::embeds::build_embed(Some(format!("{guild_description}")))
        .author(embed_author)
        .title(guild_name)
        .thumbnail(guild_icon_url)
        .image(guild_banner_url)
        .fields(embed_fields)
        .footer(embed_footer)
        .timestamp(guild_created_at);

    let reply = builders::replies::build_reply(None::<String>, &Some(embed), true);

    ctx.send(reply).await?;

    Ok(())
}
