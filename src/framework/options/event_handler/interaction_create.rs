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
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage},
};
use sqlx::SqlitePool;
use tracing::{error, warn};

use crate::{database::suggestions, Data};

pub(crate) async fn handle_create(interaction: &Interaction, ctx: &Context, data: &Data) {
    let pool = &data.pool;

    let created_at = Utc::now().naive_utc();

    match interaction {
        Interaction::Component(component) => {
            let message = &component.message;
            let message_id = message.id;

            handle_suggestion_message(component, message_id, created_at, ctx, pool).await;
            // handle_warnings_message(component, message_id, ctx, pool).await; // I don't know how to implement this shuffling bullshit.
        }
        _ => {}
    }
}

async fn handle_suggestion_message(
    component: &ComponentInteraction,
    message_id: MessageId,
    created_at: NaiveDateTime,
    ctx: &Context,
    pool: &SqlitePool,
) {
    let custom_id = &component.data.custom_id;
    let guild_id = match component.guild_id {
        Some(value) => value,
        None => {
            warn!("Couldn't get guild ID");
            return;
        }
    };
    let user_id = component.user.id;
    let owner_id = match guild_id.to_guild_cached(&ctx.cache) {
        Some(value) => value.owner_id,
        None => {
            warn!("Couldn't get guild owner ID");
            return;
        }
    };
    let channel_id = component.channel_id;

    if user_id != owner_id {
        let interaction_response_message = CreateInteractionResponseMessage::new()
            .content("Sorry, but you can't accept or reject suggestions.")
            .ephemeral(true);
        let interaction_response = CreateInteractionResponse::Message(interaction_response_message);
        let _ = component
            .create_response(&ctx.http, interaction_response)
            .await;
        return;
    }

    if custom_id == "accept_suggest" {
        let accepted_at = Utc::now().naive_utc();

        suggestions::update_suggest(
            i64::from(message_id),
            i64::from(guild_id),
            i64::from(user_id),
            i64::from(owner_id),
            created_at,
            Some(accepted_at),
            None,
            pool,
        )
        .await;
    } else if custom_id == "reject_suggest" {
        let rejected_at = Utc::now().naive_utc();

        suggestions::update_suggest(
            i64::from(message_id),
            i64::from(guild_id),
            i64::from(user_id),
            i64::from(owner_id),
            created_at,
            None,
            Some(rejected_at),
            pool,
        )
        .await;
    }

    let edit_message = EditMessage::default().components(Vec::new());
    let mut message = match channel_id.message(&ctx.http, message_id).await {
        Ok(message) => message,
        Err(why) => {
            error!("Couldn't get message: {why:?}");
            return;
        }
    };
    let _ = message.edit(&ctx.http, edit_message).await;
}
