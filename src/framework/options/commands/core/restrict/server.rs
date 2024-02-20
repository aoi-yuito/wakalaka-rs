// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::Guild;

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
    ephemeral
)]
/// Disallow a server from having yours truly.
pub(super) async fn server(
    ctx: Context<'_>,
    #[description = "The server to restrict."]
    #[rename = "server"]
    guild: Guild,
    #[min_length = 1]
    #[max_length = 255]
    #[description = "The reason for restricting."]
    reason: String,
) -> Result<(), Error> {
    let db = &ctx.data().db;

    let ctx_guild = models::guilds::guild(ctx)?;
    let ctx_guild_id = ctx_guild.id;
    let ctx_guild_name = &ctx_guild.name;

    let guild_id = guild.id;
    let guild_name = &guild.name;

    let guild_owner_id = guild.owner_id;

    if ctx_guild_id == guild_id {
        let reply = components::replies::error_reply_embed(
            format!("Cannot disallow {ctx_guild_name} from having yours truly."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let result = match queries::guilds::select_guild_id_from(db, &guild_id).await {
        Ok(_) => match queries::restricted_guilds::select_guild_id_from(db, &guild_id).await {
            Ok(_) => Err(format!(
                "{guild_name} is already disallowed from having yours truly."
            )),
            _ => {
                queries::restricted_guilds::insert_into(db, &guild_id, &reason).await?;
                queries::restricted_users::insert_into(db, &guild_owner_id, &reason).await?;

                Ok(format!(
                    "{guild_name} isn't able to have yours truly anymore."
                ))
            }
        },
        _ => Err(format!("{guild_name} isn't in the database!")),
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    guild_id.leave(ctx).await?;

    Ok(())
}
