// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Mentionable, User};
use wakalaka_core::types::{Context, Throwable};
use wakalaka_db::queries;
use wakalaka_utils::{accessors, builders};

use crate::commands;

#[poise::command(
    slash_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    required_bot_permissions = "SEND_MESSAGES",
    owners_only,
    user_cooldown = 5,
    ephemeral
)]
/// Disallow a user from using yours truly.
pub(super) async fn user(
    ctx: Context<'_>,
    #[description = "User to restrict."] user: User,
    #[min_length = 1]
    #[max_length = 255]
    #[description = "Reason for restricting."]
    reason: String,
) -> Throwable<()> {
    if let Ok(true) = commands::is_user_bot_or_system(ctx, &user).await {
        return Ok(());
    }

    let data = ctx.data();
    let db = &data.db;

    let user_id = &user.id;
    let user_created_at = &user.created_at();
    let user_mention = user.mention();

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_owner_id = &guild.owner_id;

    let result = match queries::users::fetch_user_id_from_db(db, user_id).await {
        Ok(_) if user_id == guild_owner_id => {
            Err(format!("Cannot restrict yourself from using yours truly."))
        }
        _ => {
            // In case not in database yet, add you now ...
            queries::users::add_user_to_db(db, user_id, user_created_at).await?;

            match queries::restricted_users::fetch_user_id_from_db(db, user_id).await {
                Ok(_) => Err(format!(
                    "{user_mention} is already restricted from using yours truly."
                )),
                _ => {
                    queries::restricted_users::add_restricted_user_to_db(
                        db,
                        user_id,
                        &reason,
                        user_created_at,
                    )
                    .await?;

                    Ok(format!(
                        "{user_mention} has been restricted from using yours truly."
                    ))
                }
            }
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(emsg) => builders::replies::build_warning_reply_with_embed(emsg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
