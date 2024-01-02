/**
 * Copyright (C) 2024 Kasutaja
 *
 * wakalaka-rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * wakalaka-rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with wakalaka-rs. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::util::uses::*;

pub async fn on_error(error: FrameworkError<'_, crate::Data, crate::Error>) {
    match error {
        FrameworkError::Setup { error, .. } => {
            panic!("Failed to start: {error}");
        }
        FrameworkError::Command { error, ctx, .. } => {
            let _ = ctx
                .reply(&format!("Failed to execute command: {error}"))
                .await;
        }
        FrameworkError::ArgumentParse {
            error, input, ctx, ..
        } => {
            let _ = ctx
                .reply(&format!("Failed to parse `{input:#?}`: {error}"))
                .await;
        }
        FrameworkError::CommandStructureMismatch {
            description, ctx, ..
        } => {
            let _ = ctx.reply(&*description).await;
        }
        FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => {
            let _ = ctx
                .reply(&format!(
                    "You're too fast for me! Try again in {remaining_cooldown:?} seconds."
                ))
                .await;
        }
        FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            let _ = ctx
                .reply(&format!(
                    "I'm missing the following permission(s): `{missing_permissions:?}`"
                ))
                .await;
        }
        FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            let _ = ctx
                .reply(&format!(
                    "You're missing the following permission
                    (s): `{missing_permissions:?}`"
                ))
                .await;
        }
        FrameworkError::NotAnOwner { ctx, .. } => {
            let _ = ctx
                .reply(format!("You're not my owner, {}!", ctx.author()))
                .await;
        }
        FrameworkError::GuildOnly { ctx, .. } => {
            let _ = ctx
                .reply("This command can only be used in a server.")
                .await;
        }
        FrameworkError::DmOnly { ctx, .. } => {
            let _ = ctx
                .reply("This command can only be used in a Direct Message.")
                .await;
        }
        FrameworkError::NsfwOnly { ctx, .. } => {
            let _ = ctx
                .reply("This command can only be used in a NSFW channel.")
                .await;
        }
        error => {
            if let Err(e) = builtins::on_error(error).await {
                eprintln!("Error: {e}");
            }
        }
    }
}

pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, crate::Data, crate::Error>,
    _data: &crate::Data,
) -> Result<(), crate::Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            let user_name = data_about_bot.user.name.clone();
            println!("Logged in as '{user_name}'");
        }
        FullEvent::Message { new_message } => {
            let attachments = &new_message.attachments;
            if !attachments.is_empty() {
                let mut metadata = HashMap::new();
                for (index, attachment) in attachments.iter().enumerate() {
                    FileMetadata::read(index, attachment, &mut metadata)
                        .await
                        .expect("Failed to read metadata");
                }

                println!("{:?}", metadata);
            }
        }
        _ => {}
    }
    Ok(())
}
