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
    all::{PermissionOverwrite, PermissionOverwriteType, Permissions},
    builder::{CreateActionRow, CreateMessage},
};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    database::{guilds, suggestions},
    utility::{components::buttons, components::embeds, components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Misc",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Send a suggestion to the management.
pub async fn suggest(
    ctx: Context<'_>,
    #[description = "The suggestion to send."]
    #[min_length = 32]
    #[max_length = 1024]
    message: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let message_char_count = message.chars().count();
    if message_char_count < 32 || message_char_count > 1024 {
        let reply = messages::info_reply(
            "Suggestion must be between `32` and `1024` characters long.",
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let (guild_id, guild_name) = (
        models::guilds::guild_id(ctx)?,
        models::guilds::guild_name(ctx)?,
    );

    let suggestion_channel =
        guilds::select_suggestions_channel_id_from_guilds(&guild_id, pool).await;
    if suggestion_channel.is_none() {
        error!("Couldn't find suggestion channel in {guild_name}");

        let reply = messages::info_reply(
            format!(
                "I need to be configured before suggestions could be made. Please use `/setup suggestions` to configure me."
            ),
            true,
        );
        ctx.send(reply).await?;
    } else {
        let suggestion_channel = suggestion_channel.unwrap();
        let (suggestion_channel_id, suggestion_channel_name) =
            (suggestion_channel, suggestion_channel.name(ctx).await?);

        let bot_id = ctx.cache().current_user().id;
        let bot_permissions = PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(bot_id),
        };
        if let Err(why) = suggestion_channel_id
            .create_permission(ctx, bot_permissions)
            .await
        {
            error!("Couldn't create permission overwrite for #{suggestion_channel_name}: {why:?}");
            return Err(why.into());
        }

        let (user_name, user_face) = (models::users::author_name(ctx)?, ctx.author().face());
        let (user_id, moderator_id) = (
            models::users::author_id(ctx)?,
            models::guilds::owner_id(ctx)?,
        );

        let created_at = Utc::now().naive_utc();

        let (accept_suggest, reject_suggest) = (
            buttons::accept_suggest_button(),
            buttons::reject_suggest_button(),
        );

        let embed = embeds::suggest_command_embed(user_name, user_face, &message, created_at);
        let components = CreateActionRow::Buttons(vec![accept_suggest, reject_suggest]);

        let message_builder = CreateMessage::default()
            .embed(embed)
            .components(vec![components]);

        let message = match suggestion_channel_id
            .send_message(ctx, message_builder)
            .await
        {
            Ok(value) => value,
            Err(why) => {
                error!("Couldn't send message: {why:?}");
                return Err(why.into());
            }
        };
        let message_id = message.id;

        let uuid = Uuid::new_v4().to_string();

        match suggestions::insert_into_suggestions(
            &uuid,
            i64::from(*user_id),
            i64::from(moderator_id),
            created_at,
            None,
            None,
            i64::from(message_id),
            i64::from(suggestion_channel_id),
            i64::from(guild_id),
            pool,
        )
        .await
        {
            Ok(_) => {
                info!(
                    "@{user_name} sent a suggestion to #{suggestion_channel_name} in {guild_name}"
                );

                let reply =
                    messages::ok_reply(format!("I've sent your suggestion in for review."), true);
                ctx.send(reply).await?;
            }
            Err(why) => {
                error!("Couldn't insert into Suggestions: {why:?}");
                return Err(why.into());
            }
        }
    }

    Ok(())
}
