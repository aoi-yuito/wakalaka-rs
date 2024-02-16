// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Mentionable, User};

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
/// Allow a user to use yours truly.
pub(super) async fn user(
    ctx: Context<'_>,
    #[description = "The user to unrestrict."] user: User,
) -> Result<(), Error> {
    let db = &ctx.data().db;

    if user.bot || user.system {
        let reply = components::replies::error_reply_embed(
            "Cannot allow a bot or system user to use yours truly.",
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let user_id = user.id;
    let user_mention = user.mention();

    let guild = models::guilds::guild(ctx)?;

    let guild_owner_id = guild.owner_id;
    let guild_owner_mention = guild_owner_id.mention();

    let query = queries::users::select_user_id_from(db, &user_id).await;

    let result = match query {
        Ok(_) if user_id == guild_owner_id => Err(format!(
            "Cannot allow {guild_owner_mention} to use yours truly."
        )),
        _ => {
            let restricted_query =
                queries::restricted_users::select_user_id_from(db, &user_id).await;
            match restricted_query {
                Ok(_) => {
                    queries::restricted_users::delete_from(db, &user_id).await?;

                    Ok(format!("{user_mention} is able to use yours truly again."))
                }
                _ => Err(format!(
                    "{user_mention} is already allowed to use yours truly!"
                )),
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
