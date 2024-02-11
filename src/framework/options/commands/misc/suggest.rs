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
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    database::{guilds, suggestions},
    utility::{components::buttons, components::embeds, components::messages, models},
    Context, Error,
};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Miscellaneous",
    required_bot_permissions = "SEND_MESSAGES",
    guild_only,
    user_cooldown = 5
)]
/// Make a suggestion for the server.
pub async fn suggest(
    ctx: Context<'_>,
    #[description = "The suggestion to send."]
    #[min_length = 1]
    #[max_length = 1024]
    message: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let guild_id = models::guilds::guild_id(ctx)?;
    let guild_name = models::guilds::guild_name(ctx, guild_id);

    let suggestion_channel_id =
        guilds::select_suggestions_channel_id_from_guilds(&guild_id, pool).await;
    if suggestion_channel_id.is_none() {
        warn!("Couldn't find suggestion channel in {guild_name}");

        let reply = messages::warn_reply(
            format!(
                "Yours truly must be configured before suggestions could be made. Please use `/setup suggestions` to configure yours truly."
            ),
            true,
        );
        ctx.send(reply).await?;
    } else {
        let suggestion_channel_id =
            suggestion_channel_id.expect("Failed to get suggestions channel ID");
        let suggestion_channel_name = suggestion_channel_id.name(ctx).await?;

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
            error!("Failed to create permission overwrite for #{suggestion_channel_name}: {why:?}");
            return Err(why.into());
        }

        let (user_name, user_face) = (&ctx.author().name, ctx.author().face());
        let (user_id, moderator_id) = (ctx.author().id, models::guilds::owner_id(ctx)?);

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

        let message = suggestion_channel_id
            .send_message(ctx, message_builder)
            .await?;
        let message_id = message.id;

        let uuid = Uuid::new_v4().to_string();

        suggestions::insert_into_suggestions(
            &uuid,
            i64::from(user_id),
            i64::from(moderator_id),
            created_at,
            None,
            None,
            i64::from(message_id),
            i64::from(suggestion_channel_id),
            i64::from(guild_id),
            pool,
        )
        .await?;

        info!("@{user_name} made suggestion in {guild_name}");

        let reply = messages::ok_reply(
            format!("Your suggestion has been sent in for review."),
            true,
        );
        ctx.send(reply).await?;
    }

    Ok(())
}
