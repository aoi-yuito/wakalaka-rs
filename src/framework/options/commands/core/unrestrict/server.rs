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
/// Allow a server to invite yours truly into it.
pub(super) async fn server(
    ctx: Context<'_>,
    #[description = "Server to unrestrict."]
    #[rename = "id"]
    guild_id: GuildId,
) -> Throwable<()> {
    let db = &ctx.data().db;

    let ctx_guild = models::guilds::guild(ctx)?;
    let ctx_guild_id = ctx_guild.id;

    let guild = models::guilds::guild_from_id(ctx, &guild_id)?;
    let guild_name = &guild.name;

    let guild_owner_id = guild.owner_id;

    if ctx_guild_id == guild_id {
        let reply = builders::replies::error_reply_embed(
            format!("Cannot unrestrict your own server."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let result = match queries::restricted_guilds::select_guild_id(db, &guild_id).await {
        Ok(_) => {
            queries::restricted_guilds::delete(db, &guild_id).await?;
            queries::restricted_users::delete(db, &guild_owner_id).await?;

            Ok(format!("{guild_name} has been unrestricted."))
        }
        _ => Err(format!(
            "Cannot unrestrict {guild_name} as it's unrestricted already."
        )),
    };

    let reply = match result {
        Ok(message) => builders::replies::ok_reply_embed(message, true),
        Err(message) => builders::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
