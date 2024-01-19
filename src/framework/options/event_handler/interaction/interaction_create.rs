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

use chrono::{NaiveDateTime, Utc};
use poise::serenity_prelude::Context;
use serenity::{
    all::{ComponentInteraction, Interaction, MessageId},
    builder::EditMessage,
};
use sqlx::SqlitePool;
use tracing::{error, warn};

use crate::{database::suggestions, utility::messages, Data};

pub(crate) async fn handle_create(interaction: &Interaction, ctx: &Context, data: &Data) {
    let pool = &data.pool;

    let created_at = Utc::now().naive_utc();

    match interaction {
        Interaction::Component(component) => {
            let custom_id = &component.data.custom_id;

            let message = &component.message;
            let message_id = message.id;

            if custom_id == "accept_suggest" || custom_id == "reject_suggest" {
                handle_suggestion_message(component, custom_id, message_id, created_at, ctx, pool)
                    .await;
            }
        }
        _ => {}
    }
}

async fn handle_suggestion_message(
    component: &ComponentInteraction,
    custom_id: &String,
    message_id: MessageId,
    created_at: NaiveDateTime,
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
    let (owner_id, user_id, channel_id) = (
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

    if user_id != owner_id {
        let response =
            messages::error_response("Only moderators can accept or reject suggestions.").await;
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
        suggestions::update_suggest(
            i64::from(message_id),
            i64::from(guild_id),
            i64::from(user_id),
            i64::from(owner_id),
            created_at,
            Some(now),
            None,
            pool,
        )
        .await;
    } else if custom_id == "reject_suggest" {
        suggestions::update_suggest(
            i64::from(message_id),
            i64::from(guild_id),
            i64::from(user_id),
            i64::from(owner_id),
            created_at,
            None,
            Some(now),
            pool,
        )
        .await;
    }

    let edit_message = EditMessage::default().components(Vec::new());
    if let Err(why) = message.edit(&ctx.http, edit_message).await {
        error!("Couldn't edit message: {why:?}");
        return;
    }
}
