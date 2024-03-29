// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::{
    all::{Mentionable, User},
    builder::EditMember,
};
use tracing::info;

use crate::{
    utils::{builders, models},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_NICKNAMES",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_NICKNAMES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Modify a user's nickname.
pub(super) async fn nick(
    ctx: Context<'_>,
    #[description = "User to modify nickname of."] user: User,
    #[description = "New nickname for a user."]
    #[min_length = 1]
    #[max_length = 32]
    nickname: Option<String>,
) -> Throwable<()> {
    if user.system {
        let reply =
            builders::replies::error_reply_embed("Cannot modify a system user's nickname.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let author = ctx.author();
    let author_name = &author.name;

    let user_id = user.id;
    let user_name = &user.name;
    let user_mention = user.mention();

    let nickname = nickname.unwrap_or(String::new());

    let guild = models::guilds::guild(ctx)?;
    let guild_id = guild.id;
    let guild_name = &guild.name;

    let mut member = guild_id.member(ctx, user_id).await?;

    let member_builder = EditMember::default().nickname(&nickname);

    let result = match member.edit(ctx, member_builder).await {
        Ok(_) => {
            if nickname.is_empty() {
                info!("@{author_name} reset @{user_name}'s nickname in {guild_name}");
                Ok(format!("{user_mention}'s nickname has been reset."))
            } else {
                info!("@{author_name} changed @{user_name}'s nickname to {nickname:?} in {guild_name}");
                Ok(format!(
                    "{user_mention}'s nickname has been changed to `{nickname}`."
                ))
            }
        }
        Err(why) => {
            if nickname.is_empty() {
                info!("Failed to reset @{user_name}'s nickname in {guild_name}: {why:?}");
                Err(format!(
                    "An error occurred while resetting {user_mention}'s  nickname."
                ))
            } else {
                info!("Failed to change @{user_name}'s nickname to {nickname:?} in {guild_name}: {why:?}");
                Err(format!(
                    "An error occurred while changing {user_mention}'s nickname."
                ))
            }
        }
    };

    let reply = match result {
        Ok(message) => builders::replies::ok_reply_embed(message, true),
        Err(message) => builders::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
