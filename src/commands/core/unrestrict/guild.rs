// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Guild;
use wakalaka_core::types::{Context, Throwable};
use wakalaka_db::queries;
use wakalaka_utils::builders;

#[poise::command(
    slash_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    required_bot_permissions = "SEND_MESSAGES",
    owners_only,
    user_cooldown = 5,
    ephemeral
)]
/// Allow a server to have yours truly in it.
pub(super) async fn guild(
    ctx: Context<'_>,
    #[description = "Server to unrestrict."] guild: Guild,
) -> Throwable<()> {
    let data = ctx.data();
    let db = &data.db;

    let guild_id = &guild.id;
    let guild_name = guild.name;

    let guild_owner_id = &guild.owner_id;

    let result = match queries::restricted_guilds::fetch_guild_id_from_db(db, guild_id).await {
        Ok(_) => {
            queries::restricted_guilds::remove_restricted_guild_from_db(db, guild_id).await?;

            // Simply get your tits cleaned...
            queries::restricted_users::remove_restricted_user_from_db(db, guild_owner_id).await?;

            Ok(format!(
                "{guild_name} is no longer restricted from having yours truly in it."
            ))
        }
        _ => Err(format!(
            "{guild_name} is not restricted from having yours truly in it."
        )),
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(msg) => builders::replies::build_error_reply_with_embed(msg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
