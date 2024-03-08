// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Mentionable, Role, User};
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
/// Take away a role from a user.
pub(super) async fn unassign(
    ctx: Context<'_>,
    #[description = "The role to take."] role: Role,
    #[description = "The user to take the role from."] user: User,
) -> Throwable<()> {
    let author = ctx.author();
    let author_name = &author.name;

    let user_id = user.id;
    let user_name = &user.name;
    let user_mention = user.mention();

    let role_id = role.id;
    let role_name = &role.name;
    let role_mention = role.mention();

    let guild = models::guilds::guild(ctx)?;
    let guild_id = guild.id;
    let guild_name = &guild.name;

    let member = guild_id.member(&ctx, user_id).await?;

    let result = match member.remove_role(ctx, role_id).await {
        Ok(_) => {
            info!("@{author_name} took @{role_name} from @{user_name} in {guild_name}");
            Ok(format!("Took {role_mention} away from {user_mention}."))
        }
        Err(why) => {
            error!("Failed to take @{role_name} away from @{user_name} in {guild_name}: {why:?}");
            Err(format!(
                "An error occurred while taking {role_mention} away from {user_mention}."
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
