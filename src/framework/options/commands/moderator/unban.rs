// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Mentionable, User};
use tracing::{error, info};

use crate::{
    database::queries::{self, violations::Violation},
    utils::{components, models},
    Context, Error,
};

#[poise::command(
    slash_command,
    category = "Moderator",
    required_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS | SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Lift a ban from a user.
pub(super) async fn unban(
    ctx: Context<'_>,
    #[description = "The user to unban."] user: User,
) -> Result<(), Error> {
    let db = &ctx.data().db;
    let kind = Violation::Ban;

    if user.system {
        let reply = components::replies::error_reply_embed("Cannot unban a system user.", true);

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
        let reply = components::replies::error_reply_embed("Cannot unban yourself.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    if let Err(_) = queries::users::select_user_id_from(db, &user_id).await {
        let reply = components::replies::error_reply_embed(
            format!("{user_mention} hasn't done anything yet!"),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let uuids = queries::violations::select_uuids_from(db, &kind, &guild_id, &user_id).await?;
    
    let mut violations = queries::users::select_violations_from(db, &user_id).await?;

    let result = match guild_id.unban(ctx, user_id).await {
        Ok(_) => {
            if uuids.is_empty() {
                let reply =
                    components::replies::error_reply_embed("{user_mention} is not banned!", true);

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

            info!("@{author_name} unbanned @{user_name} from {guild_name}");
            Ok(format!("{user_mention} has been unbanned."))
        }
        Err(why) => {
            error!("Failed to unban @{user_name} from {guild_name}: {why:?}");
            Err(format!(
                "An error occurred while unbanning {user_mention}."
            ))
        }
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
