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

use chrono::{NaiveDateTime, TimeZone, Utc};
use serenity::{
    all::{
        colours::branding, ButtonStyle, PermissionOverwrite, PermissionOverwriteType, Permissions,
        ReactionType,
    },
    builder::{CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateMessage},
    model::Timestamp,
};
use tracing::{error, warn};

use crate::{database::suggestions, Context, Error};

/// Suggest things for yours truly, or for community.
#[poise::command(prefix_command, slash_command, category = "Miscellaneous", guild_only)]
pub(crate) async fn suggest(
    ctx: Context<'_>,
    #[description = "Brief overview of your suggestion."] message: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let number_of_message = message.chars().count();
    if number_of_message < 32 || number_of_message > 1024 {
        let message = format!("Suggestion must be between 32 and 1024 characters.");
        let _ = ctx.reply(message).await;

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

        let accept_button = CreateButton::new("accept_button")
            .style(ButtonStyle::Success)
            .emoji(ReactionType::from('👍'))
            .label("Accept");
        let reject_button = CreateButton::new("reject_button")
            .style(ButtonStyle::Danger)
            .emoji(ReactionType::from('👎'))
            .label("Reject");

        let embed = embed(user_name, user_avatar_url, &message, created_at);
        let components = CreateActionRow::Buttons(vec![accept_button, reject_button]);

        let suggest_message = CreateMessage::default()
            .embed(embed.clone())
            .components(vec![components]);

        let message = match suggest_channel_id
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
        let message =
            format!("Sorry, but I couldn't find appropriate channel to send your suggestion to.");
        let _ = ctx.reply(message).await;
    }

    let _ = ctx.reply("Your suggestion has been sent!").await;

    Ok(())
}

fn embed(
    name: &String,
    avatar_url: String,
    description: &String,
    created_at: NaiveDateTime,
) -> CreateEmbed {
    let now = Timestamp::from(Utc.from_utc_datetime(&created_at));

    CreateEmbed::default()
        .author(embed_author(name, avatar_url))
        .description(description)
        .color(branding::BLURPLE)
        .timestamp(Timestamp::from(now))
}

fn embed_author(name: &String, avatar_url: String) -> CreateEmbedAuthor {
    CreateEmbedAuthor::new(name).icon_url(avatar_url)
}
