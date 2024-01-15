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
use poise::CreateReply;
use serenity::{
    all::{colours::branding, Member, User},
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage},
    model::Timestamp,
};
use tracing::{info, warn};

use crate::{
    database::{infractions, members},
    framework::options::commands::moderator::InfractionType,
    Context, Error,
};

/// Warns user for their misbehaviour.
#[poise::command(slash_command, required_permissions = "MODERATE_MEMBERS")]
pub(crate) async fn warn(
    ctx: Context<'_>,
    #[description = "User to give warning to."] member: Member,
    #[description = "Quick overview of your decision."] reason: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let number_of_reason = reason.chars().count();
    if number_of_reason < 6 || number_of_reason > 80 {
        let message = format!("Reason must be between 8 and 80 characters.");
        let _ = ctx.reply(message).await;

        return Ok(());
    }

    let infraction_type = InfractionType::Warn.as_str();

    let user_id_raw = member.user.id;
    let user_id = i64::from(user_id_raw);
    let user_name = &member.user.name;

    let moderator = ctx.author();
    let moderator_id_raw = moderator.id;
    let moderator_id = i64::from(moderator_id_raw);
    let moderator_name = &moderator.name;

    let guild_id_raw = match ctx.guild_id() {
        Some(guild_id) => guild_id,
        None => {
            warn!("Couldn't get guild ID");
            return Ok(());
        }
    };
    let guild_id = i64::from(guild_id_raw);
    let guild_name = match guild_id_raw.name(&ctx.cache()) {
        Some(guild_name) => guild_name,
        None => {
            warn!("Couldn't get guild name");
            return Ok(());
        }
    };

    let created_at = Utc::now().naive_utc();

    let mut infractions = match members::infractions(user_id, guild_id, pool).await {
        Some(infractions) => infractions,
        None => {
            warn!("Couldn't get infractions for @{user_name}");
            return Ok(());
        }
    };
    if infractions >= 3 {
        let message = format!("Sorry, but <@{user_id}> already has maximum number of infractions. Please take further action(s) manually.");
        let _ = ctx.reply(message).await;

        return Ok(());
    } else {
        while infractions < 3 {
            let content =
                format!("You've been warned by <@{moderator_id}> in {guild_name}: {reason}");
            let message = CreateMessage::default().content(content);
            let _ = member.user.direct_message(&ctx, message).await;

            infractions += 1;

            members::update_member(user_id, guild_id, infractions, false, false, false, pool).await;

            break;
        }

        infractions::insert_infractions(
            user_id,
            infraction_type,
            moderator_id,
            &reason,
            Some(created_at),
            None,
            true,
            pool,
        )
        .await;
        info!("@{user_name} warned by @{moderator_name}: {reason}");

        let embed = embed(
            &member,
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
    user: &Member,
    user_id: i64,
    user_name: &String,
    moderator: &User,
    moderator_id: i64,
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

fn embed_footer(user: &User, name: &String) -> CreateEmbedFooter {
    let moderator_icon_url = user.avatar_url().unwrap_or(user.default_avatar_url());

    CreateEmbedFooter::new(name).icon_url(moderator_icon_url)
}

fn embed_author(member: &Member, name: &String) -> CreateEmbedAuthor {
    let user_icon_url = member
        .user
        .avatar_url()
        .unwrap_or(member.user.default_avatar_url());

    CreateEmbedAuthor::new(name).icon_url(user_icon_url)
}
