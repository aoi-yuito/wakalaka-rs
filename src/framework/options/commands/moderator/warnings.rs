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

use chrono::{DateTime, Utc};
use poise::CreateReply;
use serenity::{
    all::{colours::branding, ButtonStyle, ReactionType, User, UserId},
    builder::{CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter},
};
use tracing::{error, warn};

use crate::{
    database::{
        infractions::{self, InfractionType},
        users,
    },
    Context, Error,
};

/// Gets a list of warnings for given user.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    guild_only
)]
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

    let warnings = match infractions::warnings(user_id, guild_id, infraction_type, pool).await {
        Ok(warnings) => warnings,
        Err(why) => {
            error!("Couldn't get warnings from database: {why:?}");
            return Ok(());
        }
    };
    let warning = match warnings.first() {
        Some(warning) => warning,
        None => {
            warn!("Couldn't get first warning from database");
            return Ok(());
        }
    };

    let case_id = warning.0;
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
    let active = &warning.6;

    let previous_case = CreateButton::new("previous_case")
        .style(ButtonStyle::Primary)
        .emoji(ReactionType::from('👈'))
        .label("Previous Case")
        .disabled(true);
    let next_case = CreateButton::new("next_case")
        .style(ButtonStyle::Primary)
        .emoji(ReactionType::from('👉'))
        .label("Next Case");

    let embed = embed(
        &case_id,
        &user,
        &user_id,
        user_name,
        &moderator_id,
        &created_at,
        reason,
        active,
    );
    let components = CreateActionRow::Buttons(vec![previous_case, next_case]);

    if infractions > 1 {
        let reply = CreateReply::default()
            .embed(embed)
            .components(vec![components]);
        let _ = ctx.send(reply).await;
    } else {
        let reply = CreateReply::default().embed(embed);
        let _ = ctx.send(reply).await;
    }

    Ok(())
}

pub(crate) fn embed(
    case_id: &i32,
    user: &User,
    user_id: &UserId,
    user_name: &String,
    moderator_id: &UserId,
    created_at: &String,
    reason: &String,
    active: &bool,
) -> CreateEmbed {
    let active_status = match active {
        true => format!("✅"),
        false => format!("❌"),
    };

    CreateEmbed::default()
        .author(embed_author(user, user_name))
        .title(format!("Case #{case_id}"))
        .field("User:", format!("<@{user_id}>"), true)
        .field("Moderator:", format!("<@{moderator_id}>"), true)
        .field("Reason:", reason, false)
        .footer(embed_footer(&active_status, created_at))
        .colour(branding::YELLOW)
}

fn embed_footer(status: &String, created_at: &String) -> CreateEmbedFooter {
    CreateEmbedFooter::new(format!("{status} {created_at}"))
}

fn embed_author(user: &User, user_name: &String) -> CreateEmbedAuthor {
    let user_icon_url = user.avatar_url().unwrap_or(user.default_avatar_url());

    CreateEmbedAuthor::new(user_name).icon_url(user_icon_url)
}
