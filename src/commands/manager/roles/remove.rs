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
/// Remove a role from a member.
pub(super) async fn remove(
    ctx: Context<'_>,
    #[description = "Role to remove."] role: Role,
    #[description = "Member to remove role from."] member: Member,
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

    let result = match member.remove_role(ctx, role_id).await {
        Ok(_) => {
            tracing::info!("@{author_name} removed @{role_name} from @{user_name} in {guild_name}");

            Ok(format!(
                "{role_mention} has been removed from {user_mention}."
            ))
        }
        Err(e) => {
            tracing::error!(
                "@{author_name} failed to remove @{role_name} from @{user_name} in {guild_name}: {e:?}"
            );

            Err(format!(
                "An error occurred while removing {role_mention} from {user_mention}."
            ))
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(emsg) => builders::replies::build_error_reply_with_embed(emsg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
