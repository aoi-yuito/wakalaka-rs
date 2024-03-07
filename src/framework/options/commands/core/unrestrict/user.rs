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
    user_cooldown = 5,
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
            "Cannot unrestrict a bot or system user from using yours truly.",
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let user_id = user.id;
    let user_mention = user.mention();

    let guild = models::guilds::guild(ctx)?;

    let guild_owner_id = guild.owner_id;

    let result = match queries::users::select_user_id_from(db, &user_id).await {
        Ok(_) if user_id == guild_owner_id => Err(format!(
            "Cannot unrestrict yourself from using yours truly."
        )),
        _ => match queries::restricted_users::select_user_id_from(db, &user_id).await {
            Ok(_) => {
                queries::restricted_users::delete_from(db, &user_id).await?;

                Ok(format!("{user_mention} has been unrestricted from using yours truly!"))
            }
            _ => Err(format!("Cannot unrestrict {user_mention} from using yours truly as they're not restricted.")),
        },
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
