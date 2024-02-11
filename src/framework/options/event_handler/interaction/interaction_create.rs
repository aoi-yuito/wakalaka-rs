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

use chrono::Utc;
use serenity::{
    all::{ComponentInteraction, Interaction, MessageId},
    builder::EditMessage,
};
use sqlx::SqlitePool;
use tracing::error;

use crate::{
    database::suggestions,
    utility::{components::messages, models},
    Data, Error,
};

pub async fn handle(
    interaction: &Interaction,
    ctx: &crate::serenity::Context,
    data: &Data,
) -> Result<(), Error> {
    let pool = &data.pool;

    match interaction {
        Interaction::Component(component) => {
            let custom_id = &component.data.custom_id;

            let message = &component.message;
            let message_id = message.id;

            if custom_id == "accept_suggest" || custom_id == "reject_suggest" {
                handle_suggestion_message(component, custom_id, message_id, ctx, pool).await?;
            }
        }
        _ => {}
    }

    Ok(())
}

async fn handle_suggestion_message(
    component: &ComponentInteraction,
    custom_id: &String,
    message_id: MessageId,
    ctx: &crate::serenity::Context,
    pool: &SqlitePool,
) -> Result<(), Error> {
    let guild_id = models::guilds::guild_id_from_component_raw(component);
    if let Ok(guild_id) = guild_id {
        let (moderator_id, user_id, channel_id) = (
            models::guilds::owner_id_raw(ctx, guild_id).await?,
            component.user.id,
            component.channel_id,
        );

        if user_id != moderator_id {
            let response = messages::error_response("Only 👑 can manage suggestions!", true).await;
            if let Err(why) = component.create_response(&ctx, response).await {
                error!("Failed to create response: {why:?}");
                return Err(why.into());
            }

            return Ok(());
        }

        let mut message = match channel_id.message(&ctx, message_id).await {
            Ok(message) => message,
            Err(why) => {
                error!("Failed to get message: {why:?}");
                return Err(why.into());
            }
        };

        let now = Utc::now().naive_utc();

        if custom_id == "accept_suggest" {
            let accept = suggestions::update_suggestions(
                i64::from(moderator_id),
                i64::from(message_id),
                i64::from(guild_id),
                Some(now),
                None,
                pool,
            )
            .await;
            if let Err(why) = accept {
                error!("Failed to accept suggestion: {why:?}");
                return Err(why.into());
            }
        } else if custom_id == "reject_suggest" {
            let reject = suggestions::update_suggestions(
                i64::from(moderator_id),
                i64::from(message_id),
                i64::from(guild_id),
                None,
                Some(now),
                pool,
            )
            .await;
            if let Err(why) = reject {
                error!("Failed to reject suggestion: {why:?}");
                return Err(why.into());
            }
        }

        let member_builder = EditMessage::default().components(Vec::new());

        if let Err(why) = message.edit(&ctx, member_builder).await {
            error!("Failed to edit message: {why:?}");
            return Err(why.into());
        }
    }

    Ok(())
}
