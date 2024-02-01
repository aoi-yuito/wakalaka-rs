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

use serenity::all::UserId;
use tracing::error;

use crate::{
    check_restricted_guild_channel,
    database::infractions::{self, InfractionType},
    utility::{
        components::embeds,
        components::{messages, replies},
        models,
    },
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Get a list of warnings for a user.
pub async fn warnings(
    ctx: Context<'_>,
    #[description = "The user to get warnings for."]
    #[rename = "user"]
    user_id: UserId,
) -> Result<(), Error> {
    let restricted_guild_channel = check_restricted_guild_channel!(ctx);
    if restricted_guild_channel {
        return Ok(());
    }

    let pool = &ctx.data().pool;

    let user = models::users::user(ctx, user_id).await?;
    if user.bot || user.system {
        let reply = messages::error_reply(
            "Sorry, but bots and system users cannot have warnings.",
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let guild_id = models::guilds::guild_id(ctx)?;

    let warnings =
        match infractions::select_from_infractions(InfractionType::Warn, &user_id, &guild_id, pool)
            .await
        {
            Ok(warnings) => warnings,
            Err(why) => {
                error!("Couldn't select warnings from infractions: {why:?}");
                return Err(why.into());
            }
        };

    let warning_count = warnings.len();
    if warning_count < 1 {
        let reply = messages::info_reply(format!("<@{user_id}> doesn't have any warnings."), true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let (uuids, moderator_ids, reasons) = (
        warnings
            .iter()
            .map(|(uuid, _, _, _, _, _, _)| uuid)
            .collect::<Vec<&String>>(),
        warnings
            .iter()
            .map(|(_, _, _, moderator_id, _, _, _)| moderator_id)
            .collect::<Vec<&i64>>(),
        warnings
            .iter()
            .map(|(_, _, _, _, reason, _, _)| reason)
            .collect::<Vec<&String>>(),
    );

    let embed = embeds::warnings_command_embed(&user, uuids, moderator_ids, reasons);

    let reply = replies::reply_embed(embed, true);
    ctx.send(reply).await?;

    Ok(())
}
