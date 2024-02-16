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
    utils::{components, models},
    Context, Error,
};

#[poise::command(
    slash_command,
    category = "Moderator",
    required_permissions = "KICK_MEMBERS",
    required_bot_permissions = "KICK_MEMBERS | SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Kick a user.
pub(super) async fn kick(
    ctx: Context<'_>,
    #[description = "The user to kick."] user: User,
    #[description = "The reason for kicking, if any."]
    #[min_length = 1]
    #[max_length = 255]
    reason: Option<String>,
) -> Result<(), Error> {
    let db = &ctx.data().db;
    let uuid = format!("{}", Uuid::new_v4());
    let kind = Violation::Kick;
    let reason = reason.unwrap_or(String::new());

    if user.system {
        let reply = components::replies::error_reply_embed("Cannot kick a system user.", true);

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
        let reply = components::replies::error_reply_embed("Cannot kick yourself.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    if let Err(_) = queries::users::select_user_id_from(db, &user_id).await {
        queries::users::insert_into(db, &user_id).await?;
    }
    if let Err(_) = queries::users::select_user_id_from(db, &author_id).await {
        queries::users::insert_into(db, &author_id).await?;
    }

    let mut violations = queries::users::select_violations_from(db, &user_id).await?;

    let member = guild_id.member(&ctx, user_id).await?;

    let result = match member.kick_with_reason(&ctx, &reason).await {
        Ok(_) => {
            let created_at = Utc::now().naive_utc();

            queries::violations::insert_into(
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

            queries::users::update_set_violations(db, &user_id, violations).await?;

            if reason.is_empty() {
                info!("@{author_name} kicked @{user_name} from {guild_name}");
                Ok(format!("{user_mention} has been kicked."))
            } else {
                info!("@{author_name} kicked @{user_name} from {guild_name}: {reason}");
                Ok(format!("{user_mention} has been kicked for {reason}."))
            }
        }
        Err(why) => {
            error!("Failed to kick @{user_name} from {guild_name}: {why:?}");
            Err(format!("An error occurred whilst kicking {user_mention}."))
        }
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
