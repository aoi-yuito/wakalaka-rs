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
use tracing::{error, warn};

use crate::{database::suggestions, serenity::Context, utility::components::messages, Data};

pub(crate) async fn handle(interaction: &Interaction, ctx: &Context, data: &Data) {
    let pool = &data.pool;

    match interaction {
        Interaction::Component(component) => {
            let custom_id = &component.data.custom_id;

            let message = &component.message;
            let message_id = message.id;

            if custom_id == "accept_suggest" || custom_id == "reject_suggest" {
                handle_suggestion_message(component, custom_id, message_id, ctx, pool).await;
            }
        }
        _ => {}
    }
}

async fn handle_suggestion_message(
    component: &ComponentInteraction,
    custom_id: &String,
    message_id: MessageId,
    ctx: &Context,
    pool: &SqlitePool,
) {
    let guild_id = match component.guild_id {
        Some(value) => value,
        None => {
            warn!("Couldn't get guild ID");
            return;
        }
    };
    let (moderator_id, user_id, channel_id) = (
        match guild_id.to_guild_cached(&ctx.cache) {
            Some(value) => value.owner_id,
            None => {
                warn!("Couldn't get guild owner ID");
                return;
            }
        },
        component.user.id,
        component.channel_id,
    );

    if user_id != moderator_id {
        let response =
            messages::error_response("Only moderators can accept or reject suggestions.", true)
                .await;
        let _ = component.create_response(&ctx.http, response).await;

        return;
    }

    let mut message = match channel_id.message(&ctx.http, message_id).await {
        Ok(message) => message,
        Err(why) => {
            error!("Couldn't get message: {why:?}");
            return;
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
            error!("Couldn't accept suggestion: {why:?}");
            return;
        }
    } else if custom_id == "reject_suggest" {
        let deny = suggestions::update_suggestions(
            i64::from(moderator_id),
            i64::from(message_id),
            i64::from(guild_id),
            None,
            Some(now),
            pool,
        )
        .await;
        if let Err(why) = deny {
            error!("Couldn't deny suggestion: {why:?}");
            return;
        }
    }

    let edit_message = EditMessage::default().components(Vec::new());
    if let Err(why) = message.edit(&ctx.http, edit_message).await {
        error!("Couldn't edit message: {why:?}");
        return;
    }
}
