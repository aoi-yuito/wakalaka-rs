// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use chrono::{Duration, Utc};
use serenity::{
    all::{Mentionable, User},
    model::Timestamp,
};
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
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "SEND_MESSAGES | MODERATE_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Put user on a time-out.
pub(super) async fn timeout(
    ctx: Context<'_>,
    #[description = "The user to time out."] user: User,
    #[description = "The amount of days a time-out should last."]
    #[min = 1]
    #[max = 28]
    time: Option<i64>,
    #[description = "The reason for timing out, if any."]
    #[min_length = 1]
    #[max_length = 255]
    reason: Option<String>,
) -> Result<(), Error> {
    let db = &ctx.data().db;
    let uuid = format!("{}", Uuid::new_v4());
    let kind = Violation::Timeout;
    let reason = reason.unwrap_or(String::new());

    if user.bot || user.system {
        let reply = components::replies::error_reply_embed(
            "Cannot put a bot or system user on a time-out.",
            true,
        );

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
        let reply =
            components::replies::error_reply_embed("Cannot put yourself on a time-out.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    if let Err(_) = queries::users::select_user_id_from(db, &user_id).await {
        queries::users::insert_into(db, &user_id).await?;
    }
    if let Err(_) = queries::users::select_user_id_from(db, &author_id).await {
        queries::users::insert_into(db, &author_id).await?;
    }

    let uuids = queries::violations::select_uuids_from(db, &kind, &guild_id, &user_id).await?;
    
    let mut violations = queries::users::select_violations_from(db, &user_id).await?;

    let mut member = guild_id.member(&ctx, user_id).await?;

    let result = if time.is_none() {
        match member.enable_communication(ctx).await {
            Ok(_) => {
                if uuids.is_empty() {
                    let reply = components::replies::error_reply_embed(
                        "{user_mention} isn't on a time-out!",
                        true,
                    );

                    ctx.send(reply).await?;

                    return Ok(());
                }

                for uuid in uuids {
                    queries::violations::delete_from(db, &uuid).await?;
                }

                violations -= 1;
                if violations < 0 {
                    violations = 0;
                }

                queries::users::update_set_violations(db, &user_id, violations).await?;

                if reason.is_empty() {
                    info!("@{author_name} got @{user_name} out of time-out in {guild_name}");
                    Ok(format!("{user_mention} has been gotten out of a time-out."))
                } else {
                    info!(
                        "@{author_name} got @{user_name} out of time-out in {guild_name}: {reason}"
                    );
                    Ok(format!(
                        "{user_mention} has been gotten out of a time-out for {reason}."
                    ))
                }
            }
            Err(why) => {
                error!("Failed to get @{user_name} out of time-out in {guild_name}: {why:?}");
                Err(format!(
                    "An error occurred whilst getting {user_mention} out of a time-out."
                ))
            }
        }
    } else {
        let time = time.unwrap_or(0);

        let now = Utc::now();
        let days = Duration::days(time);

        let timestamp = Timestamp::from(now + days);

        match member
            .disable_communication_until_datetime(ctx, timestamp)
            .await
        {
            Ok(_) => {
                if !uuids.is_empty() {
                    let reply = components::replies::error_reply_embed(
                        "{user_mention} is already on a time-out!",
                        true,
                    );

                    ctx.send(reply).await?;

                    return Ok(());
                }

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
                    info!("@{author_name} timed out @{user_name} in {guild_name}");
                    Ok(format!("{user_mention} has been timed out."))
                } else {
                    info!("@{author_name} timed out @{user_name} in {guild_name}: {reason}");
                    Ok(format!("{user_mention} has been timed out for {reason}."))
                }
            }
            Err(why) => {
                error!("Failed to time out @{user_name} in {guild_name}: {why:?}");
                Err(format!(
                    "An error occurred whilst timing out {user_mention}."
                ))
            }
        }
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
