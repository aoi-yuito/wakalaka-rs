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
use poise::CreateReply;
use serenity::{
    all::{
        colours::branding, ButtonStyle, PermissionOverwrite, PermissionOverwriteType, Permissions,
        ReactionType,
    },
    builder::{
        CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateMessage, EditMessage,
    },
    model::Timestamp,
};
use tracing::{error, warn};

use crate::{database::suggestions, Context, Error};

/// Suggest things for yours truly, or for community.
#[poise::command(slash_command)]
pub(crate) async fn suggest(
    ctx: Context<'_>,
    #[description = "Brief overview of your suggestion."] suggestion: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let number_of_suggestions = suggestion.chars().count();
    if number_of_suggestions < 10 || number_of_suggestions > 120 {
        let message_ = format!("Suggestion must be between 10 and 120 characters.");
        let _ = ctx.reply(message_).await;

        return Ok(());
    }

    let guild_id = match ctx.guild_id() {
        Some(value) => value,
        None => {
            warn!("Couldn't get guild ID");
            return Ok(());
        }
    };

    let guild_channels = match ctx.http().get_channels(guild_id).await {
        Ok(value) => value,
        Err(why) => {
            return Err(format!("Couldn't get channels in guild: {why:?}").into());
        }
    };

    let suggest_channel = guild_channels
        .iter()
        .find(|channel| channel.name == "suggestions");
    if let Some(channel) = suggest_channel {
        let suggest_channel_id = channel.id;

        let bot_id = ctx.cache().current_user().id;

        let permissions = PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(bot_id),
        };

        if let Err(why) = suggest_channel_id
            .create_permission(&ctx.http(), permissions)
            .await
        {
            return Err(
                format!("Couldn't create permission overwrite for #suggestions: {why:?}").into(),
            );
        }

        let (user_name, user_avatar_url) = (
            &ctx.author().name,
            match ctx.author().avatar_url() {
                Some(url) => url,
                None => ctx.author().default_avatar_url(),
            },
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

        let embed = embed(user_name, user_avatar_url, &suggestion);

        let accept_button = CreateButton::new("accept_button")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::from('👍'))
            .label("Accept");
        let reject_button = CreateButton::new("reject_button")
            .style(ButtonStyle::Danger)
            .emoji(ReactionType::from('👎'))
            .label("Reject");

        let components = CreateActionRow::Buttons(vec![accept_button, reject_button]);

        let suggest_message = CreateMessage::default()
            .embed(embed.clone())
            .components(vec![components]);
        let mut message = match suggest_channel_id
            .send_message(&ctx.http(), suggest_message)
            .await
        {
            Ok(value) => value,
            Err(why) => {
                error!("Couldn't send message: {why:?}");
                return Ok(());
            }
        };
        let message_id = message.id;

        let _ = ctx.reply("Your suggestion has been sent!").await;

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

        let manager = ctx.framework().shard_manager.clone();

        let shard_ids = manager.shards_instantiated().await;
        let shard_id = shard_ids[0];

        let runners = manager.runners.lock().await;
        let runner_info = match runners.get(&shard_id) {
            Some(value) => value,
            None => {
                warn!("Couldn't get shard runner");
                return Ok(());
            }
        };

        let messenger = runner_info.runner_tx.clone();
        let interaction = match message.await_component_interactions(messenger).next().await {
            Some(value) => value,
            None => {
                warn!("Couldn't get interaction");
                return Ok(());
            }
        };
        if interaction.user.id != owner_id {
            let reply = CreateReply {
                content: Some("Sorry, but you can't accept or reject suggestions.".to_string()),
                ephemeral: Some(true),
                ..Default::default()
            };
            let _ = ctx.send(reply).await;

            return Ok(());
        }

        let button_id = interaction.data.custom_id;

        let decision = match button_id.as_str() {
            "accept_button" => (true, false),
            "reject_button" => (false, true),
            other => {
                error!("Couldn't get ID for button: {other:?}");
                return Ok(());
            }
        };
        if decision.0 {
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
        } else {
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

        let edit_message = EditMessage::default().embed(embed).components(vec![]);
        message.edit(&ctx.http(), edit_message).await?;
    } else {
        let message_ =
            format!("Sorry, but I couldn't find appropriate channel to send your suggestion to.");
        let _ = ctx.reply(message_).await;
    }

    Ok(())
}

fn embed(name: &String, avatar_url: String, description: &String) -> CreateEmbed {
    CreateEmbed::default()
        .author(embed_author(name, avatar_url))
        .description(description)
        .color(branding::BLURPLE)
        .timestamp(Timestamp::from(Utc::now()))
}

fn embed_author(user_name: &String, user_avatar_url: String) -> CreateEmbedAuthor {
    CreateEmbedAuthor::new(user_name).icon_url(user_avatar_url)
}
