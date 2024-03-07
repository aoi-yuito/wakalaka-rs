// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::GuildId;

use crate::{
    database::queries,
    utils::{components, models},
    Context, Error,
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
/// Allow a server to have yours truly in it.
pub(super) async fn server(
    ctx: Context<'_>,
    #[description = "The server to unrestrict."]
    #[rename = "id"]
    guild_id: GuildId,
) -> Result<(), Error> {
    let db = &ctx.data().db;

    let ctx_guild = models::guilds::guild(ctx)?;
    let ctx_guild_id = ctx_guild.id;

    let guild = models::guilds::guild_from_id(ctx, &guild_id)?;
    let guild_name = &guild.name;

    let guild_owner_id = guild.owner_id;

    if ctx_guild_id == guild_id {
        let reply = components::replies::error_reply_embed(
            format!("Cannot unrestrict your own server from having yours truly in it."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let result = match queries::restricted_guilds::select_guild_id_from(db, &guild_id).await {
        Ok(_) => {
            queries::restricted_guilds::delete_from(db, &guild_id).await?;
            queries::restricted_users::delete_from(db, &guild_owner_id).await?;

            Ok(format!("{guild_name} has been unrestricted from having yours truly in it."))
        }
        _ => Err(format!(
            "Cannot unrestrict {guild_name} from having yours truly in it as it's unrestricted already."
        )),
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
