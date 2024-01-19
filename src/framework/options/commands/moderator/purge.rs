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

use serenity::{all::Message, builder::GetMessages};
use tracing::{error, info};

use crate::{utility::messages, Context, Error};

/// Delete a given amount of messages.
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("after", "any", "around", "before"),
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    guild_only,
    subcommand_required,
    ephemeral
)]
pub(crate) async fn purge(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Delete a given amount of messages after a specific message.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    guild_only,
    ephemeral
)]
pub(crate) async fn after(
    ctx: Context<'_>,
    #[description = "ID of the message to delete after"] message: Message,
    #[description = "The amount to delete after. (1-100)"] count: Option<u8>,
) -> Result<(), Error> {
    let count = count.unwrap_or(1);
    if count < 1 || count > 100 {
        let reply = messages::warn_reply("Amount must be between 1 and 100 message(s).");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let http = ctx.serenity_context().http.clone(); // Why?
    let channel_id = ctx.channel_id();
    let user_name = ctx.author().name.clone();

    let handle = tokio::spawn(async move {
        let mut number_of_deleted_messages = 0;

        let channel_name = match channel_id.name(&http).await {
            Ok(channel_name) => channel_name,
            Err(why) => {
                error!("Couldn't get channel name: {why:?}");
                return number_of_deleted_messages;
            }
        };

        let message_id = message.id;

        let messages_after = GetMessages::default().after(message_id).limit(count);
        let messages = match channel_id.messages(&http, messages_after).await {
            Ok(messages) => messages,
            Err(why) => {
                error!("Couldn't get messages: {why:?}");
                return number_of_deleted_messages;
            }
        };
        for message in messages {
            if let Err(why) = message.delete(&http).await {
                error!("Couldn't delete message: {why:?}");
                continue;
            }

            number_of_deleted_messages += 1;
        }

        info!("@{user_name} deleted {number_of_deleted_messages} message(s) in #{channel_name}");

        number_of_deleted_messages
    });

    let reply_before = messages::reply("Deleting message(s)...");
    let reply = ctx.send(reply_before).await?;

    let number_of_deleted_messages = handle.await.unwrap_or(0);

    let reply_after =
        messages::ok_reply(format!("Deleted {number_of_deleted_messages} message(s).",));
    reply.edit(ctx, reply_after).await?;

    Ok(())
}

// Delete a given amount of messages.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    guild_only,
    ephemeral
)]
pub(crate) async fn any(
    ctx: Context<'_>,
    #[description = "The amount to delete. (1-100)"] count: Option<u8>,
) -> Result<(), Error> {
    let count = count.unwrap_or(1);
    if count < 1 || count > 100 {
        let reply = messages::warn_reply("Amount must be between 1 and 100 message(s).");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let http = ctx.serenity_context().http.clone(); // Why?
    let channel_id = ctx.channel_id();
    let user_name = ctx.author().name.clone();

    let handle = tokio::spawn(async move {
        let mut number_of_deleted_messages = 0;

        let channel_name = match channel_id.name(&http).await {
            Ok(channel_name) => channel_name,
            Err(why) => {
                error!("Couldn't get channel name: {why:?}");
                return number_of_deleted_messages;
            }
        };

        let messages_any = GetMessages::default().limit(count);

        let messages = match channel_id.messages(&http, messages_any).await {
            Ok(messages) => messages,
            Err(why) => {
                error!("Couldn't get messages: {why:?}");
                return number_of_deleted_messages;
            }
        };
        for message in messages {
            if let Err(why) = message.delete(&http).await {
                error!("Couldn't delete message: {why:?}");
                continue;
            }

            number_of_deleted_messages += 1;
        }

        info!("@{user_name} deleted {number_of_deleted_messages} message(s) in #{channel_name}");

        number_of_deleted_messages
    });

    let reply_before = messages::reply("Deleting message(s)...");
    let reply = ctx.send(reply_before).await?;

    let number_of_deleted_messages = handle.await.unwrap_or(0);

    let reply_after =
        messages::ok_reply(format!("Deleted {number_of_deleted_messages} message(s).",));
    reply.edit(ctx, reply_after).await?;

    Ok(())
}

/// Delete a given amount of messages around a specific message.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    guild_only,
    ephemeral
)]
pub(crate) async fn around(
    ctx: Context<'_>,
    #[description = "ID of the message to delete around."] message: Message,
    #[description = "The amount to delete around. (1-100)"] count: Option<u8>,
) -> Result<(), Error> {
    let count = count.unwrap_or(1);
    if count < 1 || count > 100 {
        let reply = messages::warn_reply("Amount must be between 1 and 100 message(s).");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let http = ctx.serenity_context().http.clone(); // Why?
    let channel_id = ctx.channel_id();
    let user_name = ctx.author().name.clone();

    let handle = tokio::spawn(async move {
        let mut number_of_deleted_messages = 0;

        let channel_name = match channel_id.name(&http).await {
            Ok(channel_name) => channel_name,
            Err(why) => {
                error!("Couldn't get channel name: {why:?}");
                return number_of_deleted_messages;
            }
        };

        let message_id = message.id;

        let messages_around = GetMessages::default().around(message_id).limit(count);
        let messages = match channel_id.messages(&http, messages_around).await {
            Ok(messages) => messages,
            Err(why) => {
                error!("Couldn't get messages: {why:?}");
                return number_of_deleted_messages;
            }
        };
        for message in messages {
            if let Err(why) = message.delete(&http).await {
                error!("Couldn't delete message: {why:?}");
                continue;
            }

            number_of_deleted_messages += 1;
        }

        info!("@{user_name} deleted {number_of_deleted_messages} message(s) in #{channel_name}");

        number_of_deleted_messages
    });

    let reply_before = messages::reply("Deleting message(s)...");
    let reply = ctx.send(reply_before).await?;

    let number_of_deleted_messages = handle.await.unwrap_or(0);

    let reply_after =
        messages::ok_reply(format!("Deleted {number_of_deleted_messages} message(s).",));
    reply.edit(ctx, reply_after).await?;

    Ok(())
}

/// Delete a given amount of messages before a specific message.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MANAGE_MESSAGES",
    guild_only,
    ephemeral
)]
pub(crate) async fn before(
    ctx: Context<'_>,
    #[description = "ID of the message to delete before."] message: Message,
    #[description = "The amount of to delete before. (1-100)"] count: Option<u8>,
) -> Result<(), Error> {
    let count = count.unwrap_or(1);
    if count < 1 || count > 100 {
        let reply = messages::warn_reply("Amount must be between 1 and 100 message(s).");
        if let Err(why) = ctx.send(reply).await {
            error!("Couldn't send reply: {why:?}");
        }

        return Ok(());
    }

    let http = ctx.serenity_context().http.clone(); // Why?
    let channel_id = ctx.channel_id();
    let user_name = ctx.author().name.clone();

    let handle = tokio::spawn(async move {
        let mut number_of_deleted_messages = 0;

        let channel_name = match channel_id.name(&http).await {
            Ok(channel_name) => channel_name,
            Err(why) => {
                error!("Couldn't get channel name: {why:?}");
                return number_of_deleted_messages;
            }
        };

        let message_id = message.id;

        let messages_before = GetMessages::default().before(message_id).limit(count);
        let messages = match channel_id.messages(&http, messages_before).await {
            Ok(messages) => messages,
            Err(why) => {
                error!("Couldn't get messages: {why:?}");
                return number_of_deleted_messages;
            }
        };
        for message in messages {
            if let Err(why) = message.delete(&http).await {
                error!("Couldn't delete message: {why:?}");
                continue;
            }

            number_of_deleted_messages += 1;
        }

        info!("@{user_name} deleted {number_of_deleted_messages} message(s) in #{channel_name}");

        number_of_deleted_messages
    });

    let reply_before = messages::reply("Deleting message(s)...");
    let reply = ctx.send(reply_before).await?;

    let number_of_deleted_messages = handle.await.unwrap_or(0);

    let reply_after =
        messages::ok_reply(format!("Deleted {number_of_deleted_messages} message(s).",));
    reply.edit(ctx, reply_after).await?;

    Ok(())
}
