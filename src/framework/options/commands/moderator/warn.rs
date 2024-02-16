// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use chrono::Utc;
use serenity::all::{Mentionable, User};
use tracing::info;
use uuid::Uuid;

use crate::{
    database::queries::{self, violations::Violation},
    utils::{components, models},
    Context, Error,
};

#[poise::command(
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "SEND_MESSAGES | MODERATE_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Give user a warning.
pub(super) async fn warn(
    ctx: Context<'_>,
    #[description = "The user to warn."] user: User,
    #[description = "The reason for warning."]
    #[min_length = 3]
    #[max_length = 120]
    reason: String,
) -> Result<(), Error> {
    let db = &ctx.data().db;

    let kind = Violation::Warning;

    if user.bot || user.system {
        let reply = components::replies::error_reply_embed(
            "Cannot give warning to a bot or system user.",
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let author = ctx.author();
    let author_id = author.id;
    let author_name = &author.name;
    let author_mention = author.mention();

    let user_id = user.id;
    let user_name = &user.name;
    let user_mention = user.mention();

    let guild = models::guilds::guild(ctx)?;
    let guild_id = guild.id;
    let guild_name = &guild.name;

    if user_id == author_id {
        let reply = components::replies::error_reply_embed("Cannot give yourself a warning.", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    if let Err(_) = queries::users::select_user_id_from(db, &user_id).await {
        queries::users::insert_into(db, &user_id).await?;
    }

    let mut violations = queries::users::select_violations_from(db, &user_id).await?;

    let uuid = Uuid::new_v4();
    let uuids = queries::violations::select_uuids_from(db, &kind, &guild_id, &user_id).await?;

    let uuid_count = uuids.len();
    if uuid_count >= 3 {
        let reply = components::replies::error_reply_embed(
            format!("Cannot give more than {uuid_count} warnings to {user_mention}."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let created_at = Utc::now().naive_utc();

    let result = match queries::violations::insert_into(
        db,
        &uuid,
        &kind,
        &guild_id,
        &user_id,
        &author_id,
        &reason,
        &created_at,
    )
    .await
    {
        Ok(_) => {
            violations += 1;

            queries::users::update_set_violations(db, &user_id, violations).await?;

            let message = components::messages::message_embed(format!(
                "You've been warned by {author_mention} in {guild_name} for {reason}.",
            ));

            user.dm(ctx, message).await?;

            info!("@{author_name} warned @{user_name} in {guild_name}: {reason}");
            Ok(format!("{user_mention} has been warned for {reason}."))
        }
        Err(_) => Err(format!("An error occurred whilst warning {user_mention}.")),
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
