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

    let guild_name = &guild.name;
    let guild_id = &guild.id;
    let guild_created_at = &guild_id.created_at();

    let guild_owner_id = &guild.owner_id;
    let guild_owner_created_at = &guild_owner_id.created_at();

    let result = match queries::guilds::fetch_guild_id_from_db(db, guild_id).await {
        Ok(db_guild_id) if *guild_id == db_guild_id => Err(format!(
            "Cannot restrict your own server from having yours truly in it."
        )),
        _ => {
            if let Ok(_) = queries::restricted_guilds::fetch_guild_id_from_db(db, guild_id).await {
                Err(format!(
                    "{guild_name} is already restricted from having yours truly in it."
                ))
            } else {
                queries::restricted_guilds::add_restricted_guild_to_db(
                    db,
                    guild_id,
                    &reason,
                    guild_created_at,
                )
                .await?;

                // Oh yeah, the owner of this server can go fuck a pair of tits...
                queries::restricted_users::add_restricted_user_to_db(
                    db,
                    guild_owner_id,
                    &reason,
                    guild_owner_created_at,
                )
                .await?;

                Ok(format!(
                    "{guild_name} has been restricted from having yours truly in it."
                ))
            }
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(msg) => builders::replies::build_error_reply_with_embed(msg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
