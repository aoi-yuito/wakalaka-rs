// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{EditMember, Mentionable, User};
use tracing::{error, info};
use wakalaka_core::types::{Context, Throwable};
use wakalaka_utils::{accessors, builders};

use crate::commands;

#[poise::command(
    slash_command,
    category = "Manager",
    required_permissions = "MANAGE_NICKNAMES",
    required_bot_permissions = "SEND_MESSAGES | MANAGE_NICKNAMES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Edit a user's nickname.
pub(super) async fn nick(
    ctx: Context<'_>,
    #[description = "User to edit nickname of."] user: User,
    #[description = "Nickname to set, if any."]
    #[min_length = 1]
    #[max_length = 32]
    nickname: Option<String>,
) -> Throwable<()> {
    // API dislikes changing bot or system user nicknames...
    if commands::is_user_bot_or_system(ctx, &user).await? {
        return Ok(());
    }

    let author = ctx.author();
    let author_id = author.id;
    let author_name = &author.name;

    let user_id = user.id;
    let user_name = &user.name;
    let user_mention = user.mention();

    let nickname = nickname.unwrap_or(String::new()); // Without providance of nickname, is of no more.

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_name = &guild.name;

    // ...and so does dislike changing own nickname...
    if user_id == author_id {
        error!("@{author_name} failed to set own nickname to {nickname:?} in {guild_name}");

        let reply =
            builders::replies::build_error_reply_with_embed("Cannot edit your own nickname.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let mut member = guild.member(ctx, user_id).await?.into_owned();

    let edited_member = EditMember::default().nickname(&nickname);

    let result = match member.edit(ctx, edited_member).await {
        Ok(_) => {
            if nickname.is_empty() {
                info!("@{author_name} reset {user_name}'s nickname in {guild_name}");

                Ok(format!("{user_mention}'s nickname has been reset."))
            } else {
                info!("@{author_name} set {user_name}'s nickname to {nickname:?} in {guild_name}");

                Ok(format!(
                    "{user_mention}'s nickname has been set to `{nickname}`."
                ))
            }
        }
        Err(e) => {
            if nickname.is_empty() {
                error!(
                    "@{author_name} failed to reset {user_name}'s nickname in {guild_name}: {e:?}"
                );

                Err(format!(
                    "An error occurred while resetting {user_mention}'s nickname."
                ))
            } else {
                error!(
                    "@{author_name} failed to set {user_name}'s nickname to {nickname:?} in {guild_name}: {e:?}"
                );

                Err(format!(
                    "An error occurred while setting {user_mention}'s nickname to `{nickname}`."
                ))
            }
        }
    };

    let reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true),
        Err(msg) => builders::replies::build_error_reply_with_embed(msg, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
