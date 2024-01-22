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
    database::infractions::{self, InfractionType},
    utility::{components::embeds, components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    guild_only,
    ephemeral
)]
/// Get a list of warnings for a user.
pub(crate) async fn warnings(
    ctx: Context<'_>,
    #[description = "The user to get warnings for."]
    #[rename = "user"]
    user_id: UserId,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user = models::users::user(ctx, user_id).await;
    if user.bot || user.system {
        let reply = messages::error_reply(
            "Sorry, but bots and system users cannot have warnings.",
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let guild_id = models::guilds::guild_id(ctx).await;

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

    let number_of_warnings = warnings.len();
    if number_of_warnings < 1 {
        let reply = messages::warn_reply(
            format!("I'm afraid <@{user_id}> doesn't have any warnings."),
            true,
        );
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
            return Err(why.into());
        }

        return Ok(());
    }

    let (uuids, moderator_ids, reasons) = (
        warnings
            .iter()
            .map(|(uuid, _, _, _, _, _, _)| uuid.clone())
            .collect::<Vec<String>>(),
        warnings
            .iter()
            .map(|(_, _, _, moderator_id, _, _, _)| moderator_id.clone())
            .collect::<Vec<i64>>(),
        warnings
            .iter()
            .map(|(_, _, _, _, reason, _, _)| reason.clone())
            .collect::<Vec<String>>(),
    );

    let embed = embeds::warnings_embed(&user, uuids, moderator_ids, reasons);

    let reply = messages::reply_embed(embed, true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
        return Err(why.into());
    }

    Ok(())
}
