// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use poise::CreateReply;
use serenity::{
    all::GuildId,
    builder::{CreateEmbedAuthor, CreateEmbedFooter},
};

use crate::{
    utils::{builders, models},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Information",
    required_bot_permissions = "SEND_MESSAGES | EMBED_LINKS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Get information about a server yours truly is in.
pub(super) async fn server(
    ctx: Context<'_>,
    #[description = "The server to get information of."]
    #[rename = "id"]
    guild_id: GuildId,
) -> Throwable<()> {
    let guild = models::guilds::guild_from_id(ctx, &guild_id)?;
    let guild_name = &guild.name;
    let guild_description = guild.description.as_deref().unwrap_or_default();
    let guild_icon_url = guild.icon_url().unwrap_or_default();
    let guild_banner_url = guild.banner_url().unwrap_or_default();

    let guild_owner_id = guild.owner_id;
    let guild_owner = guild_owner_id.to_user(ctx).await?;
    let guild_owner_name = &guild_owner.name;
    let guild_owner_face = guild_owner.face();

    let guild_role_count = guild.roles.len();
    let guild_member_count = guild
        .members
        .iter()
        .filter(|member| !member.1.user.bot)
        .count();
    let guild_channel_count = guild.channels.len();

    let guild_created_at = guild_id.created_at();

    let embed_author = CreateEmbedAuthor::new(guild_owner_name).icon_url(guild_owner_face);
    let embed_fields = vec![
        ("🛡️ Roles", format!("{guild_role_count}"), true),
        ("👥 Members", format!("{guild_member_count}"), true),
        ("🌐 Channels", format!("{guild_channel_count}"), true),
    ];
    let embed_footer = CreateEmbedFooter::new(format!("🆔{guild_id}"));

    let embed = builders::embeds::embed(guild_description)
        .author(embed_author)
        .title(guild_name)
        .thumbnail(guild_icon_url)
        .image(guild_banner_url)
        .fields(embed_fields)
        .footer(embed_footer)
        .timestamp(guild_created_at);

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}
