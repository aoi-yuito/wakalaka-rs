// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::GuildId;

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
/// Disallow a server from inviting yours truly into it.
pub(super) async fn server(
    ctx: Context<'_>,
    #[description = "Server to restrict."]
    #[rename = "id"]
    guild_id: GuildId,
    #[min_length = 1]
    #[max_length = 255]
    #[description = "Reason for restricting."]
    reason: String,
) -> Throwable<()> {
    let db = &ctx.data().db;

    let ctx_guild = models::guilds::guild(ctx)?;
    let ctx_guild_id = ctx_guild.id;

    let guild = models::guilds::guild_from_id(ctx, &guild_id)?;
    let guild_name = &guild.name;

    let guild_owner_id = guild.owner_id;

    if ctx_guild_id == guild_id {
        let reply =
            builders::replies::error_reply_embed(format!("Cannot restrict your own server."), true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let result = match queries::restricted_guilds::select_guild_id(db, &guild_id).await {
        Ok(_) => Err(format!(
            "Cannot restrict {guild_name} as it's restricted already."
        )),
        _ => {
            queries::restricted_guilds::insert(db, &guild_id, &reason).await?;
            queries::restricted_users::insert(db, &guild_owner_id, &reason).await?;

            Ok(format!("{guild_name} has been restricted: {reason}"))
        }
    };

    let reply = match result {
        Ok(message) => builders::replies::ok_reply_embed(message, true),
        Err(message) => builders::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    guild_id.leave(ctx).await?;

    Ok(())
}
