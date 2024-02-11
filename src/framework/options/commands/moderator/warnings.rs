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

use serenity::all::{Mentionable, User};

use crate::{
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
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Get a list of warnings for a user.
pub async fn warnings(
    ctx: Context<'_>,
    #[description = "The user to get warnings for."] user: User,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    if user.bot || user.system {
        let reply = messages::error_reply("Cannot warn bots and system users!", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    let (user_id, user_mention) = (user.id, user.mention());

    let guild_id = models::guilds::guild_id(ctx)?;

    let warnings =
        infractions::select_from_infractions(InfractionType::Warn, &user_id, &guild_id, pool)
            .await?;

    let warning_count = warnings.len();
    if warning_count < 1 {
        let reply =
            messages::warn_reply(format!("{user_mention} doesn't have any warnings!"), true);
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
