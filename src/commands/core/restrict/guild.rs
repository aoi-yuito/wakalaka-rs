// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Guild;
use wakalaka_core::types::{Context, Throwable};
use wakalaka_db::queries;
use wakalaka_utils::{accessors, builders};

#[poise::command(
    slash_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    required_bot_permissions = "SEND_MESSAGES",
    owners_only,
    user_cooldown = 5,
    ephemeral
)]
/// Disallow a server from having yours truly in it.
pub(super) async fn guild(
    ctx: Context<'_>,
    #[description = "Server to restrict."] guild: Guild,
    #[min_length = 1]
    #[max_length = 255]
    #[description = "Reason for restricting."]
    reason: String,
) -> Throwable<()> {
    let data = ctx.data();
    let db = &data.db;

    let ctx_guild = accessors::guilds::fetch_guild(ctx)?;
    let ctx_guild_id = ctx_guild.id;

    let guild_name = &guild.name;
    let guild_id = &guild.id;

    let guild_owner_id = &guild.owner_id;
    let guild_owner_created_at = &guild_owner_id.created_at();

    let result = match queries::guilds::fetch_guild_id_from_db(db, guild_id).await {
        Ok(_) if *guild_id == ctx_guild_id => Err(format!(
            "Cannot restrict your own server from having yours truly in it."
        )),
        _ => {
            let guild_created_at = &guild_id.created_at();

            match queries::restricted_guilds::fetch_guild_id_from_db(db, guild_id).await {
                Ok(_) => Err(format!(
                    "{guild_name:?} is already restricted from having yours truly in it."
                )),
                _ => {
                    // Too afraid of Discord shutting down my application over bad servers doing bad stuffs.
                    queries::restricted_guilds::add_restricted_guild_to_db(
                        db,
                        guild_id,
                        &reason,
                        guild_created_at,
                    )
                    .await?;

                    // さよなら！
                    queries::restricted_users::add_restricted_user_to_db(
                        db,
                        guild_owner_id,
                        &reason,
                        guild_owner_created_at,
                    )
                    .await?;

                    Ok(format!(
                        "{guild_name:?} has been restricted from having yours truly in it."
                    ))
                }
            }
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(msg) => builders::replies::build_warning_reply_with_embed(msg, true),
    };

    ctx.send(reply).await?;

    guild.leave(ctx).await?;

    Ok(())
}
