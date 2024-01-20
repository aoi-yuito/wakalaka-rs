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

use poise::CreateReply;
use serenity::all::UserId;
use tracing::{error, warn};

use crate::{
    database::{
        infractions::{self, InfractionType},
        users,
    },
    utility::{self, embeds, messages},
    Context, Error,
};

/// Get a list of warnings for a user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    guild_only,
    ephemeral
)]
pub(crate) async fn warnings(
    ctx: Context<'_>,
    #[description = "The user to get warnings for."]
    #[rename = "user"]
    user_id: UserId,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user = utility::user(user_id, ctx).await;
    if user.bot || user.system {
        let reply = messages::error_reply("Cannot get warnings for a bot or system user.");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let user_name = &user.name;

    let guild_id = utility::guild_id(ctx);

    let warn_type = InfractionType::Warn.as_str();

    let user_infractions = match users::infractions(user_id, guild_id, pool).await {
        Some(infractions) => infractions,
        None => {
            warn!("Couldn't get infractions for @{user_name} in database");
            return Ok(());
        }
    };

    let warnings = match infractions::infractions(user_id, guild_id, warn_type, pool).await {
        Ok(warnings) => warnings,
        Err(why) => {
            error!("Couldn't get warnings from database: {why:?}");
            return Ok(());
        }
    };

    let number_of_warnings = warnings.len();

    // There's a failsafe for if the user doesn't have any entries in the database but has a fucking infraction anyway. Fucking how you ever cause the latter to happen is beyond me...
    if user_infractions < 1 || number_of_warnings < 1 {
        let reply = messages::warn_reply(format!("<@{user_id}> doesn't have any warnings."));
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let case_ids = warnings
        .iter()
        .map(|warning| warning.0)
        .collect::<Vec<i32>>();
    let moderator_ids = warnings
        .iter()
        .map(|warning| warning.2)
        .collect::<Vec<i64>>();
    let reasons = warnings
        .iter()
        .map(|warning| warning.3.clone())
        .collect::<Vec<String>>();

    let embed = embeds::warnings_embed(case_ids, &user, &user_name, moderator_ids, reasons);

    let reply = CreateReply::default().embed(embed).ephemeral(true);
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
    }

    Ok(())
}
