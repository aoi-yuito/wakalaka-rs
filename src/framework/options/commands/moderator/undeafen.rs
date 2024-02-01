// Copyright (C) 2024 Kawaxte
//
// wakalaka-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// wakalaka-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.

use serenity::{all::UserId, builder::EditMember};
use tracing::{error, info};

use crate::{
    check_restricted_guild_channel,
    database::{
        guild_members,
        infractions::{self, InfractionType},
        users,
    },
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "DEAFEN_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Allow a user to interact in voice channels.
pub async fn undeafen(
    ctx: Context<'_>,
    #[description = "The user to undeafen."]
    #[rename = "user"]
    user_id: UserId,
    #[description = "The reason for undeafening, if any."]
    #[min_length = 6]
    #[max_length = 80]
    reason: Option<String>,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let pool = &ctx.data().pool;

    let user = models::users::user(ctx, user_id).await?;
    let (user_name, user_mention) = (&user.name, models::users::user_mention(ctx, user_id).await?);

    let moderator = models::users::author(ctx)?;
    let (moderator_id, moderator_name) = (moderator.id, &moderator.name);

    if user_id == moderator_id {
        let reply = messages::error_reply("Sorry, but you cannot undeafen yourself.", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let guild_id = models::guilds::guild_id(ctx)?;

    let mut user_infractions = users::select_infractions_from_users(&user_id, pool).await?;
    if user_infractions < 1 {
        let reply =
            messages::info_reply(format!("{user_mention} hasn't been punished before."), true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let deafens =
        infractions::select_from_infractions(InfractionType::Deaf, &user_id, &guild_id, pool)
            .await?;
    for deafen in deafens {
        let uuid = deafen.0;

        let mut member = models::members::member(ctx, guild_id, user_id).await?;
        let member_builder = EditMember::default().deafen(false);

        if let Err(why) = member.edit(ctx, member_builder).await {
            error!("Couldn't undeafen @{user_name}: {why:?}");

            let reply = messages::error_reply(
                format!("Sorry, but I couldn't undeafen {user_mention}."),
                true,
            );
            ctx.send(reply).await?;

            return Err(why.into());
        }

        guild_members::update_guilds_members_set_deaf(&user_id, false, pool).await?;

        if let Some(ref reason) = reason {
            let reason_char_count = reason.chars().count();
            if reason_char_count < 6 || reason_char_count > 80 {
                let reply = messages::info_reply(
                    "Reason must be between `6` and `80` characters long.",
                    true,
                );
                ctx.send(reply).await?;

                return Ok(());
            }

            info!("@{user_name} undeafened by @{moderator_name}: {reason}");
        } else {
            info!("@{user_name} undeafened by @{moderator_name}")
        }

        infractions::delete_from_infractions(&uuid, &guild_id, pool).await?;

        user_infractions -= 1;
        if user_infractions < 0 {
            user_infractions = 0;
        }

        users::update_users_set_infractions(&user_id, user_infractions, pool).await?;

        let reply = messages::ok_reply(format!("{user_mention} has been undeafened."), true);
        ctx.send(reply).await?;
    }

    Ok(())
}
