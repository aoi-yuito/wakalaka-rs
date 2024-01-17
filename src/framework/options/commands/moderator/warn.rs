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

use chrono::{Duration, NaiveDateTime, TimeZone, Utc};
use poise::CreateReply;
use serenity::{
    all::{colours::branding, User, UserId},
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage},
    model::Timestamp,
};
use tracing::{info, warn};

use crate::{
    database::{
        infractions::{self, InfractionType},
        users,
    },
    Context, Error,
};

/// Warns user for their misbehaviour.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    guild_only
)]
pub(crate) async fn warn(
    ctx: Context<'_>,
    #[description = "User to give warning to."] user: User,
    #[description = "Quick overview of your decision."] reason: String,
) -> Result<(), Error> {
    // Why would you ever try this to begin with?
    if user.bot || user.system {
        let message = format!("Sorry, but bot(s) and system user(s) can't be warned.");
        let _ = ctx.reply(message).await;

        return Ok(());
    }

    let pool = &ctx.data().pool;

    let number_of_reason = reason.chars().count();
    if number_of_reason < 6 || number_of_reason > 80 {
        let message = format!("Reason must be between 8 and 80 characters.");
        let _ = ctx.reply(message).await;

        return Ok(());
    }

    let infraction_type = InfractionType::Warn.as_str();

    let user_id = user.id;
    let user_name = &user.name;

    let moderator = ctx.author();
    let moderator_id = moderator.id;
    let moderator_name = &moderator.name;

    let guild_id = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            warn!("Couldn't get guild ID");
            return Ok(());
        }
    };
    let guild_name = match guild_id.name(&ctx.cache()) {
        Some(guild_name) => guild_name,
        None => {
            warn!("Couldn't get guild name");
            return Ok(());
        }
    };

    let created_at = Utc::now().naive_utc();
    let expires_at = match Utc
        .from_utc_datetime(&created_at)
        .checked_add_signed(Duration::weeks(3))
    {
        Some(expires_at) => expires_at.naive_utc(),
        None => {
            warn!("Couldn't get expiration date");
            return Ok(());
        }
    };

    let mut infractions = match users::infractions(user_id, guild_id, pool).await {
        Some(infractions) => infractions,
        None => {
            warn!("Couldn't get infractions for @{user_name}");
            return Ok(());
        }
    };

    // Why should you ever have more than 3 warnings?
    if infractions >= 3 {
        let message = format!("Sorry, but <@{user_id}> already has maximum number of infractions. Please take further action(s) manually.");
        let _ = ctx.reply(message).await;

        return Ok(());
    } else {
        while infractions < 3 {
            let content =
                format!("You've been warned by <@{moderator_id}> in {guild_name}: {reason}");
            let message = CreateMessage::default().content(content);
            let _ = user.direct_message(&ctx, message).await;

            infractions += 1;

            users::update_user(user_id, guild_id, infractions, false, false, false, pool).await;

            break;
        }

        infractions::insert_infraction(
            user_id,
            infraction_type,
            moderator_id,
            guild_id,
            &reason,
            Some(created_at),
            Some(expires_at),
            true,
            pool,
        )
        .await;
        info!("@{user_name} warned by @{moderator_name}: {reason}");

        let embed = embed(
            &user,
            user_id,
            user_name,
            moderator,
            moderator_id,
            moderator_name,
            reason,
            created_at,
        );

        let message = CreateReply::default().embed(embed);
        let _ = ctx.send(message).await;
    }

    Ok(())
}

fn embed(
    user: &User,
    user_id: UserId,
    user_name: &String,
    moderator: &User,
    moderator_id: UserId,
    moderator_name: &String,
    reason: String,
    created_at: NaiveDateTime,
) -> CreateEmbed {
    let now = Timestamp::from(Utc.from_utc_datetime(&created_at));

    CreateEmbed::default()
        .author(embed_author(user, user_name))
        .title("⚠️ You've been warned! ⚠️")
        .field("User:", format!("<@{user_id}>"), true)
        .field("Moderator:", format!("<@{moderator_id}>"), true)
        .field("Reason:", reason, false)
        .footer(embed_footer(moderator, moderator_name))
        .timestamp(now)
        .colour(branding::YELLOW)
}

fn embed_footer(user: &User, user_name: &String) -> CreateEmbedFooter {
    let moderator_icon_url = user.avatar_url().unwrap_or(user.default_avatar_url());

    CreateEmbedFooter::new(user_name).icon_url(moderator_icon_url)
}

fn embed_author(user: &User, user_name: &String) -> CreateEmbedAuthor {
    let user_icon_url = user.avatar_url().unwrap_or(user.default_avatar_url());

    CreateEmbedAuthor::new(user_name).icon_url(user_icon_url)
}
