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
use tracing::{error, warn};

use crate::{
    database::suggestions,
    utility::{buttons, embeds, messages},
    Context, Error,
};

/// Send a suggestion of your choice for review.
#[poise::command(prefix_command, slash_command, category = "Misc", guild_only)]
pub(crate) async fn suggest(
    ctx: Context<'_>,
    #[description = "The suggestion to send. (32-1024)"] message: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let number_of_message = message.chars().count();
    if number_of_message < 32 || number_of_message > 1024 {
        let reply = messages::warn_reply("Suggestion must be between 32 and 1024 characters.");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let guild_id = match ctx.guild_id() {
        Some(value) => value,
        None => {
            warn!("Couldn't get guild ID");
            return Ok(());
        }
    };
    let guild_name = match guild_id.name(&ctx.cache()) {
        Some(value) => value,
        None => {
            warn!("Couldn't get guild name");
            return Ok(());
        }
    };

    let guild_channels = match ctx.http().get_channels(guild_id).await {
        Ok(value) => value,
        Err(why) => {
            error!("Couldn't get channels in {guild_name}: {why:?}");
            return Ok(());
        }
    };

    let suggest_channel = guild_channels
        .iter()
        .find(|channel| channel.name == "suggestions");
    if let Some(channel) = suggest_channel {
        let channel_id = channel.id;
        let channel_name = &channel.name;

        let bot_id = ctx.cache().current_user().id;
        let bot_permissions = PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(bot_id),
        };
        if let Err(why) = channel_id
            .create_permission(&ctx.http(), bot_permissions)
            .await
        {
            error!("Couldn't create permission overwrite for #{channel_name}: {why:?}");
            return Ok(());
        }

        let (user_name, user_avatar_url) = (
            &ctx.author().name,
            ctx.author()
                .avatar_url()
                .unwrap_or(ctx.author().default_avatar_url()),
        );

        let user_id = ctx.author().id;
        let owner_id = {
            let guild = match ctx.guild() {
                Some(value) => value,
                None => {
                    warn!("Couldn't get guild");
                    return Ok(());
                }
            };
            guild.owner_id
        };
        let created_at = Utc::now().naive_utc();

        let (accept_suggest, reject_suggest) = (
            buttons::accept_suggest_button(),
            buttons::reject_suggest_button(),
        );

        let embed = embeds::suggest_embed(user_name, user_avatar_url, &message, created_at);
        let components = CreateActionRow::Buttons(vec![accept_suggest, reject_suggest]);

        let suggest_message = CreateMessage::default()
            .embed(embed.clone())
            .components(vec![components]);

        let message = match channel_id.send_message(&ctx.http(), suggest_message).await {
            Ok(value) => value,
            Err(why) => {
                error!("Couldn't send message: {why:?}");
                return Ok(());
            }
        };
        let message_id = message.id;

        suggestions::insert_suggest(
            i64::from(message_id),
            i64::from(guild_id),
            i64::from(user_id),
            i64::from(owner_id),
            created_at,
            None,
            None,
            pool,
        )
        .await;
    } else {
        let reply = messages::error_reply("Couldn't find `#suggestions` channel.");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }
    }

    let reply = messages::ok_reply(format!("Suggestion has been sent in for review."));
    if let Err(why) = ctx.send(reply).await {
        error!("Couldn't send reply: {why:?}");
    }

    Ok(())
}
