// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{Member, Mentionable, Role};

use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::{accessors, builders};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_ROLES",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_ROLES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Add a role to a member.
pub(super) async fn add(
    ctx: Context<'_>,
    #[description = "Role to add."] role: Role,
    #[description = "Member to add role to."] member: Member,
) -> Throwable<()> {
    let author = ctx.author();
    let author_name = &author.name;

    let role_id = role.id;
    let role_name = &role.name;
    let role_mention = role.mention();

    let user = &member.user;
    let user_name = &user.name;
    let user_mention = user.mention();

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = &guild.name;

    let result = match member.add_role(ctx, role_id).await {
        Ok(_) => {
            tracing::info!("@{author_name} added @{role_name} to @{user_name} in {guild_name}");

            Ok(format!("{role_mention} has been added to {user_mention}."))
        }
        Err(e) => {
            tracing::error!(
                "@{author_name} failed to add @{role_name} to @{user_name} in {guild_name}: {e:?}"
            );

            Err(format!(
                "An error occurred while adding {role_mention} to {user_mention}."
            ))
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(msg) => builders::replies::build_error_reply_with_embed(msg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
