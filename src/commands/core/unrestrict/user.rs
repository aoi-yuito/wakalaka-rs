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
/// Allow a user to using yours truly.
pub(super) async fn user(
    ctx: Context<'_>,
    #[description = "User to unrestrict."] user: User,
) -> Throwable<()> {
    if let Ok(true) = commands::is_user_bot_or_system(ctx, &user).await {
        return Ok(());
    }

    let data = ctx.data();
    let db = &data.db;

    let user_id = &user.id;
    let user_mention = user.mention();

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_owner_id = &guild.owner_id;

    let result = match queries::users::fetch_user_id_from_db(db, user_id).await {
        Ok(_) if user_id == guild_owner_id => Err(format!(
            "Cannot unrestrict yourself from using yours truly."
        )),
        _ => match queries::restricted_users::fetch_user_id_from_db(db, user_id).await {
            Ok(_) => {
                queries::restricted_users::remove_restricted_user_from_db(db, user_id).await?;

                Ok(format!(
                    "{user_mention} is no longer restricted from using yours truly."
                ))
            }
            _ => Err(format!(
                "{user_mention} is not restricted from using yours truly."
            )),
        },
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(emsg) => builders::replies::build_warning_reply_with_embed(emsg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
