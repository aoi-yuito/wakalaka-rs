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
use tracing::info;

use crate::{
    database::{restricted_users, users},
    utility::{components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    user_cooldown = 5,
    ephemeral
)]
/// Allow a user to use yours truly.
pub async fn user(
    ctx: Context<'_>,
    #[description = "The user to allow usage for."]
    #[rename = "user"]
    other_user_id: UserId,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user_name = other_user_id.to_user(&ctx).await?.name;

    let owner_id = models::guilds::owner_id(ctx)?;
    if owner_id == other_user_id {
        let reply = messages::error_reply(
            format!("Cannot unrestrict 👑 from using yours truly!"),
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let failsafe_query = users::select_user_id_from_users(&other_user_id, &pool).await;
    let result = match failsafe_query {
        Some(_) if other_user_id == owner_id => {
            Err(format!("👑 is already allowed to use yours truly."))
        }
        None => Err(format!("{user_name} isn't part of yours truly!")),
        _ => {
            let previous_query =
                restricted_users::select_user_id_from_restricted_users(&other_user_id, &pool).await;
            match previous_query {
                Ok(_) => {
                    info!("Allowed usage of yours truly for {user_name}.");
                    restricted_users::delete_from_restricted_users(&other_user_id, &pool).await?;
                    Ok(format!("Allowed {user_name} to use yours truly."))
                }
                _ => Err(format!("Usage for {user_name} is already allowed!")),
            }
        }
    };

    let reply = match result {
        Ok(message) => messages::ok_reply(message, true),
        Err(message) => messages::error_reply(message, true),
    };
    ctx.send(reply).await?;

    Ok(())
}
