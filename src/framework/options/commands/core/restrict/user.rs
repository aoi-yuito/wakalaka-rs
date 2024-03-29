// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Mentionable, User};

use crate::{
    database::queries,
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
/// Disallow a user from using yours truly.
pub(super) async fn user(
    ctx: Context<'_>,
    #[description = "User to restrict."] user: User,
    #[min_length = 1]
    #[max_length = 255]
    #[description = "Reason for restricting."]
    reason: String,
) -> Throwable<()> {
    let db = &ctx.data().db;

    if user.bot || user.system {
        let reply =
            builders::replies::error_reply_embed("Cannot restrict a bot or system user.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let user_id = user.id;
    let user_mention = user.mention();

    let guild = models::guilds::guild(ctx)?;

    let guild_owner_id = guild.owner_id;

    let result = match queries::users::select_user_id(db, &user_id).await {
        Ok(_) if user_id == guild_owner_id => Err(format!("Cannot restrict yourself.")),
        _ => {
            queries::users::insert(db, &user_id).await?;

            match queries::restricted_users::select_user_id(db, &user_id).await {
                Ok(_) => Err(format!(
                    "Cannot restrict {user_mention} as they're restricted already."
                )),
                _ => {
                    queries::restricted_users::insert(db, &user_id, &reason).await?;

                    Ok(format!("{user_mention} has been restricted: {reason}"))
                }
            }
        }
    };

    let reply = match result {
        Ok(message) => builders::replies::ok_reply_embed(message, true),
        Err(message) => builders::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
