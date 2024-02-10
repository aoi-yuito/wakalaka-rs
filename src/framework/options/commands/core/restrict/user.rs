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
/// Deny a user from using yours truly.
pub async fn user(
    ctx: Context<'_>,
    #[description = "The user to deny usage for."]
    #[rename = "user"]
    other_user_id: UserId,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user_name = models::users::user_name(ctx, other_user_id).await?;

    let owner_id = models::guilds::owner_id(ctx)?;
    if other_user_id == owner_id {
        let reply = messages::error_reply(
            format!("Sorry, but you can't restrict yourself from using me."),
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let failsafe_query = users::select_user_id_from_users(&other_user_id, &pool).await;
    let result = match failsafe_query {
        Some(user_id) if user_id == owner_id => {
            Err(format!("Sorry, but I can't deny usage for {user_name}."))
        }
        _ => {
            let previous_query =
                restricted_users::select_user_id_from_restricted_users(&other_user_id, &pool).await;
            match previous_query {
                Err(_) => {
                    info!("Denied usage for {user_name}.");
                    restricted_users::insert_into_restricted_users(&other_user_id, &pool).await?;
                    Ok(format!("Denied {user_name} from using me."))
                }
                _ => Err(format!("Usage for {user_name} is already denied.")),
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
