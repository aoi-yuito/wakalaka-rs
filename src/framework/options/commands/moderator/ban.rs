// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use chrono::Utc;
use serenity::all::{Mentionable, User};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    database::queries::{self, violations::Violation},
    utils::{builders, models},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Moderator",
    required_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS | SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Ban a user.
pub(super) async fn ban(
    ctx: Context<'_>,
    #[description = "The user to ban."] user: User,
    #[description = "The number of days of messages to delete."]
    #[min = 0]
    #[max = 7]
    days: u8,
    #[description = "The reason for banning, if any."]
    #[min_length = 1]
    #[max_length = 255]
    reason: Option<String>,
) -> Throwable<()> {
    let db = &ctx.data().db;
    let kind = Violation::Ban;
    let uuid = format!("{}", Uuid::new_v4());
    let created_at = Utc::now().naive_utc();
    let reason = reason.unwrap_or(String::new());

    if user.system {
        let reply = builders::replies::error_reply_embed("Cannot ban a system user.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let author = ctx.author();
    let author_id = author.id;
    let author_name = &author.name;

    let user_id = user.id;
    let user_name = &user.name;
    let user_mention = user.mention();

    let guild = models::guilds::guild(ctx)?;
    let guild_id = guild.id;
    let guild_name = &guild.name;

    if user_id == author_id {
        let reply = builders::replies::error_reply_embed("Cannot ban yourself.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    if let Err(_) = queries::users::select_user_id(db, &user_id).await {
        queries::users::insert(db, &user_id).await?;
    }
    if let Err(_) = queries::users::select_user_id(db, &author_id).await {
        queries::users::insert(db, &author_id).await?;
    }

    let mut violations = queries::users::select_violations(db, &user_id).await?;

    let member = guild_id.member(&ctx, user_id).await?;

    let result = match member.ban_with_reason(&ctx, days, &reason).await {
        Ok(_) => {
            queries::violations::insert(
                db,
                &uuid,
                &kind,
                &guild_id,
                &user_id,
                &author_id,
                &reason,
                &created_at,
            )
            .await?;

            violations += 1;

            queries::users::update_violations(db, &user_id, violations).await?;

            if reason.is_empty() {
                info!("@{author_name} banned @{user_name} from {guild_name}");
                Ok(format!("{user_mention} has been banned!"))
            } else {
                info!("@{author_name} banned @{user_name} from {guild_name}: {reason}");
                Ok(format!("{user_mention} has been banned: {reason}"))
            }
        }
        Err(why) => {
            error!("Failed to ban @{user_name} from {guild_name}: {why:?}");
            Err(format!("An error occurred while banning {user_mention}."))
        }
    };

    let reply = match result {
        Ok(message) => builders::replies::ok_reply_embed(message, true),
        Err(message) => builders::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
