// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Mentionable, Role};
use tracing::{error, info};

use crate::{
    utils::{components, models},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_ROLES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Delete an existing role.
pub(super) async fn remove(
    ctx: Context<'_>,
    #[description = "The role to delete."] mut role: Role,
) -> Throwable<()> {
    let author = ctx.author();
    let author_name = &author.name;

    let role_name = role.name.clone();
    let role_mention = role.mention();

    let guild = models::guilds::guild(ctx)?;
    let guild_name = &guild.name;

    let result = match role.delete(ctx).await {
        Ok(_) => {
            info!("@{author_name} deleted @{role_name} from {guild_name}");
            Ok(format!("{role_mention} has been deleted."))
        }
        Err(why) => {
            error!("@{author_name} failed to delete @{role_name} from {guild_name}: {why:?}");
            Err(format!("An error occurred while deleting {role_mention}."))
        }
    };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
