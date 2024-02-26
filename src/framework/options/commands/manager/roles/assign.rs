// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Mentionable, Role, User};
use tracing::{error, info};

use crate::{
    utils::{components, models},
    Context, Error,
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
/// Give user a role.
pub(super) async fn assign(
    ctx: Context<'_>,
    #[description = "The role to give."] role: Role,
    #[description = "The user to give the role to."] user: User,
) -> Result<(), Error> {
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

    let result = match member.add_role(ctx, role_id).await {
        Ok(_) => {
            info!("@{author_name} gave @{role_name} to @{user_name} in {guild_name}");
            Ok(format!("Gave {role_mention} to {user_mention}."))
        }
        Err(why) => {
            error!("Failed to give @{role_name} to @{user_name} in {guild_name}: {why:?}");
            Err(format!(
                "An error occurred while adding {role_mention} to {user_mention}."
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
