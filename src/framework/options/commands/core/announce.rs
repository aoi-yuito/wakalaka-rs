// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use chrono::Utc;
use serenity::all::{CreateEmbedFooter, CreateMessage, Timestamp};

use crate::{
    utils::{builders, models},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    required_bot_permissions = "SEND_MESSAGES",
    owners_only,
    user_cooldown = 5,
    ephemeral
)]
/// Send an announcement to servers yours truly is in.
pub(super) async fn announce(
    ctx: Context<'_>,
    #[description = "The message to announce."]
    #[min_length = 1]
    #[max_length = 4096]
    message: String,
) -> Throwable<()> {
    let message = message.replace("  ", "\n\n");

    let author = ctx.author();
    let author_id = author.id;
    let author_name = &author.name;
    let author_face = author.face();

    let guild_ids = ctx.cache().guilds();

    let guild_id_count = guild_ids.len() - 1; // Don't count our own server.

    for guild_id in guild_ids {
        let guild = models::guilds::guild_from_id(ctx, &guild_id)?;

        let guild_owner_id = guild.owner_id;
        if author_id == guild_owner_id {
            // Don't announce to yourself!
            continue;
        }
        let guild_owner = guild_owner_id.to_user(ctx).await?;

        let now = Utc::now();

        let created_at = Timestamp::from(now);

        let embed_footer = CreateEmbedFooter::new(author_name).icon_url(&author_face);

        let embed = builders::embeds::embed(&message)
            .title("📢")
            .footer(embed_footer)
            .timestamp(created_at);

        let message = CreateMessage::default().embed(embed);

        guild_owner.dm(ctx, message).await?;
    }

    let reply = if guild_id_count == 1 {
        builders::replies::ok_reply_embed(
            format!("Announcement to {guild_id_count} server has been sent!"),
            true,
        )
    } else {
        builders::replies::ok_reply_embed(
            format!("Announcement to {guild_id_count} servers has been sent!"),
            true,
        )
    };

    ctx.send(reply).await?;

    Ok(())
}
