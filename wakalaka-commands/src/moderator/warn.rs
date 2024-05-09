// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Mentionable, User};
use uuid::Uuid;

use wakalaka_core::{
    accessors, builders,
    types::{Context, Throwable},
};
use wakalaka_database::queries;

#[poise::command(
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "SEND_MESSAGES | MODERATE_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Warn a user.
pub(super) async fn warn(
    ctx: Context<'_>,
    #[description = "User to warn."] user: User,
    #[description = "Reason for warning."]
    #[min_length = 1]
    #[max_length = 255]
    reason: String,
) -> Throwable<()> {
    if crate::is_user_bot_or_system(ctx, &user).await? {
        return Ok(());
    }

    let data = ctx.data();
    let db = &data.db;

    let author = ctx.author();
    let author_id = &author.id;
    let author_name = &author.name;
    let author_created_at = &author.created_at();

    let user_id = &user.id;
    let user_name = &user.name;
    let user_mention = user.mention();
    let user_created_at = &user.created_at();

    if user_id == author_id {
        let reply = builders::replies::build_error_reply_with_embed("Cannot warn yourself.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_id = &guild.id;
    let guild_name = &guild.name;

    // In case not in database yet...
    if let Err(_) = queries::users::fetch_user_id_from_db(db, user_id).await {
        queries::users::add_user_to_db(db, user_id, user_created_at).await?;
    }
    if let Err(_) = queries::users::fetch_user_id_from_db(db, author_id).await {
        queries::users::add_user_to_db(db, author_id, author_created_at).await?;
    }

    let uuid = &Uuid::new_v4();
    let uuids = queries::warnings::gather_all_uuids_from_db(db, guild_id, user_id).await?;

    let uuid_count = uuids.len();

    let mut warns = queries::users::fetch_warnings_from_db(db, user_id).await?;
    if warns >= 3 && uuid_count >= 3 {
        let reply = builders::replies::build_error_reply_with_embed(
            format!("Cannot warn {user_mention} anymore."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let result =
        match queries::warnings::add_warning_to_db(db, uuid, guild_id, user_id, author_id, &reason)
            .await
        {
            Ok(_) => {
                warns += 1;

                queries::users::update_warnings_in_db(db, user_id, warns).await?;

                let message = builders::messages::build_warning_message_with_embed(format!(
                    "You have been warned in {guild_name} for `{reason}`."
                ));

                user.dm(&ctx, message).await?;

                tracing::info!("@{author_name} warned @{user_name} in {guild_name}: {reason}");

                Ok(format!("{user_mention} has been warned for `{reason}`."))
            }
            Err(_) => {
                tracing::error!(
                    "@{author_name} failed to warn @{user_name} in {guild_name}: {reason}"
                );

                Err(format!(
                    "An error occurred while warning {user_mention} for `{reason}`."
                ))
            }
        };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(emsg) => builders::replies::build_error_reply_with_embed(emsg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
