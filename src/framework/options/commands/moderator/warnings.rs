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
use poise::CreateReply;
use serenity::{
    all::{colours::branding, User, UserId},
    builder::{CreateEmbed, CreateEmbedAuthor},
};
use tracing::{error, warn};

use crate::{
    database::{infractions, users},
    framework::options::commands::moderator::InfractionType,
    Context, Error,
};

/// Gets a list of warnings for given user.
#[poise::command(slash_command, required_permissions = "MODERATE_MEMBERS")]
pub(crate) async fn warnings(
    ctx: Context<'_>,
    #[description = "User to get warnings for."] user: User,
) -> Result<(), Error> {
    // Again, why would you ever try this to begin with?
    if user.bot || user.system {
        let message = format!("Sorry, but bot(s) and system user(s) can't have warnings.");
        let _ = ctx.reply(message).await;

        return Ok(());
    }

    let pool = &ctx.data().pool;

    let user_id = user.id;

    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            warn!("Couldn't get guild ID");
            return Ok(());
        }
    };

    let infractions = match users::infractions(user_id, guild_id, pool).await {
        Some(infractions) => infractions,
        None => {
            warn!("Couldn't get infractions");
            return Ok(());
        }
    };
    if infractions == 0 {
        let message = format!("Sorry, but <@{user_id}> doesn't have any warnings.");
        let _ = ctx.reply(message).await;

        return Ok(());
    }

    let infraction_type = InfractionType::Warn.as_str();

    let warnings = match infractions::warnings(user_id, infraction_type, pool).await {
        Ok(warnings) => warnings,
        Err(why) => {
            error!("Couldn't get warnings from database: {why:?}");
            return Ok(());
        }
    };
    for warning in warnings {
        let case_id = warning.0;

        let user_id = UserId::from(u64::from(user_id));
        let user = match user_id.to_user(&ctx).await {
            Ok(user) => user,
            Err(why) => {
                error!("Couldn't get user from database: {why:?}");
                return Ok(());
            }
        };
        let user_name = &user.name;
        let moderator_id = UserId::from(warning.2 as u64);
        let reason = &warning.3;
        let created_at = DateTime::<Utc>::from_naive_utc_and_offset(warning.4, Utc)
            .format("%b %d, %Y %H:%M:%S")
            .to_string();
        let expires_at = DateTime::<Utc>::from_naive_utc_and_offset(warning.5, Utc)
            .format("%b %d, %Y %H:%M:%S")
            .to_string();
        let active = &warning.6;

        let embed = embed(
            &case_id,
            &user,
            user_name,
            &moderator_id,
            &created_at,
            &expires_at,
            reason,
            active,
        );

        let reply = CreateReply::default().embed(embed);
        let _ = ctx.send(reply).await;
    }

    Ok(())
}

fn embed(
    case_id: &i32,
    user: &User,
    user_name: &String,
    moderator_id: &UserId,
    created_at: &String,
    expires_at: &String,
    reason: &String,
    active: &bool,
) -> CreateEmbed {
    CreateEmbed::default()
        .author(embed_author(user, user_name))
        .field("Case:", format!("`{case_id}`"), true)
        .field("Moderator:", format!("<@{moderator_id}>"), true)
        .field("Reason:", reason, false)
        .field("Created at:", created_at, true)
        .field("Expires at:", expires_at, true)
        .field("Active:", format!("{active}"), false)
        .colour(branding::YELLOW)
}

fn embed_author(user: &User, user_name: &String) -> CreateEmbedAuthor {
    let user_icon_url = user.avatar_url().unwrap_or(user.default_avatar_url());

    CreateEmbedAuthor::new(user_name).icon_url(user_icon_url)
}
