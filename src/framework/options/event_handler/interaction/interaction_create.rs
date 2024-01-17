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

use chrono::{DateTime, NaiveDateTime, Utc};
use poise::serenity_prelude::Context;
use serenity::{
    all::{ComponentInteraction, Interaction, MessageId, UserId},
    builder::{
        CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage,
    },
};
use sqlx::SqlitePool;
use tracing::{error, warn};

use crate::{
    database::{
        infractions::{self, InfractionType},
        suggestions,
    },
    utility::{buttons, embeds},
    Data,
};

pub(crate) async fn handle_create(interaction: &Interaction, ctx: &Context, data: &Data) {
    let pool = &data.pool;

    let created_at = Utc::now().naive_utc();

    match interaction {
        Interaction::Component(component) => {
            let message = &component.message;
            let message_id = message.id;

            handle_suggestion_message(component, message_id, created_at, ctx, pool).await;
            handle_warnings_message(component, message_id, ctx, pool).await;
        }
        _ => {}
    }
}

async fn handle_warnings_message(
    component: &ComponentInteraction,
    message_id: MessageId,
    ctx: &Context,
    pool: &SqlitePool,
) {
    let custom_id = &component.data.custom_id;
    let user_id = component.user.id;
    let guild_id = match component.guild_id {
        Some(value) => value,
        None => {
            warn!("Couldn't get guild ID");
            return;
        }
    };

    let infraction_type = InfractionType::Warn.as_str();

    let mut message = match component.channel_id.message(&ctx.http, message_id).await {
        Ok(message) => message,
        Err(why) => {
            error!("Couldn't get message: {why:?}");
            return;
        }
    };

    let warnings = match infractions::warnings(user_id, guild_id, infraction_type, pool).await {
        Ok(warnings) => warnings,
        Err(why) => {
            error!("Couldn't get warnings from database: {why:?}");
            return;
        }
    };

    let current_warning_index = message.content.parse::<usize>().unwrap_or(0);
    let number_of_warnings = warnings.len();

    let warning_index = match custom_id.as_str() {
        "next_warning" => {
            if (current_warning_index + 1) < number_of_warnings {
                current_warning_index + 1
            } else {
                current_warning_index
            }
        }
        "previous_warning" => {
            if current_warning_index > 0 {
                current_warning_index - 1
            } else {
                current_warning_index
            }
        }
        _ => current_warning_index,
    };

    let warning = &warnings[warning_index];

    let case_id = warning.0;
    let case_ids = warnings.iter().map(|warning| warning.0).collect::<Vec<_>>();
    let user = match user_id.to_user(&ctx).await {
        Ok(user) => user,
        Err(why) => {
            error!("Couldn't get user from database: {why:?}");
            return;
        }
    };
    let user_name = &user.name;
    let moderator_id = UserId::from(warning.2 as u64);
    let reason = &warning.3;
    let created_at = DateTime::<Utc>::from_naive_utc_and_offset(warning.4, Utc)
        .format("%b %d, %Y %H:%M:%S")
        .to_string();
    let active = &warning.6;

    let (mut previous_warning, mut next_warning) = (
        buttons::previous_warning_button(false),
        buttons::next_warning_button(false),
    );

    let embed = embeds::warnings_embed(
        &case_id,
        &user,
        &user_id,
        user_name,
        &moderator_id,
        &created_at,
        reason,
        active,
    );

    if custom_id == "next_warning" {
        if case_id == case_ids[number_of_warnings - 1] {
            next_warning = buttons::next_warning_button(true);
        }
    } else if custom_id == "previous_warning" {
        if case_id == case_ids[0] {
            previous_warning = buttons::previous_warning_button(true);
        }
    }

    let components = CreateActionRow::Buttons(vec![previous_warning, next_warning]);

    let edit_message = EditMessage::default()
        .content(format!("{warning_index}")) // For some tittyfuckin' retarded reason, removing this line causes the shuffling to not work after the second shuffle. What the fuck?
        .embeds(vec![embed])
        .components(vec![components]);
    let _ = message.edit(&ctx.http, edit_message).await;
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
