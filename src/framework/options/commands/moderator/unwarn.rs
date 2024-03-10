// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::time::Duration;

use serenity::{
    all::{ComponentInteractionDataKind, Mentionable, ReactionType, User},
    builder::{
        CreateActionRow, CreateInteractionResponse, CreateSelectMenu, CreateSelectMenuKind,
        CreateSelectMenuOption,
    },
};
use tokio::time::timeout;
use tracing::{error, info};

use crate::{
    database::queries::{self, violations::Violation},
    utils::{components, models},
    Context, Throwable,
};

#[poise::command(
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "SEND_MESSAGES | MODERATE_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Remove a warning from a user.
pub(super) async fn unwarn(
    ctx: Context<'_>,
    #[description = "The user to unwarn."] user: User,
) -> Throwable<()> {
    let db = &ctx.data().db;
    let kind = Violation::Warning;

    if user.bot || user.system {
        let reply = components::replies::error_reply_embed(
            "Cannot remove a warning from a bot or system user.",
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let author = ctx.author();
    let author_id = author.id;
    let author_name = &author.name;

    let user_id = user.id;
    let user_name = &user.name;
    let user_mention = user.mention();

    let guild = models::guilds::guild(ctx)?;
    let guild_id = guild.id;
    let guild_name = &guild.name;

    if user_id == author_id {
        let reply =
            components::replies::error_reply_embed("Cannot remove a warning from yourself.", true);
        ctx.send(reply).await?;

        return Ok(());
    }

    if let Err(_) = queries::users::select_user_id(db, &user_id).await {
        let reply = components::replies::error_reply_embed(
            format!("{user_mention} is not in the database!"),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let uuids = queries::violations::select_uuids(db, &kind, &guild_id, &user_id).await?;
    if uuids.is_empty() {
        let reply = components::replies::error_reply_embed(
            format!("{user_mention} does not have any warnings!"),
            true,
        );
        ctx.send(reply).await?;

        return Ok(());
    }

    let warning = queries::violations::select(db, &kind, &guild_id, &user_id).await?;

    let mut violations = queries::users::select_violations(db, &user_id).await?;

    let menu_options = warning
        .iter()
        .enumerate()
        .map(|(_, (uuid, reason, created_at))| {
            let formatted_created_at = created_at.format("%b %d, %Y").to_string();

            CreateSelectMenuOption::new(format!("@{author_name} ({formatted_created_at})"), uuid)
                .description(reason.trim())
                .emoji(ReactionType::Unicode(format!("⚠️")))
        })
        .collect::<Vec<_>>();
    let menu_kind = CreateSelectMenuKind::String {
        options: menu_options,
    };
    let menu = CreateSelectMenu::new("warning_select", menu_kind)
        .min_values(1)
        .max_values(1);

    let action_row = CreateActionRow::SelectMenu(menu);

    let reply = components::replies::reply("Which warning would you like to remove?", true)
        .components(vec![action_row]);

    let message = ctx.send(reply).await?.into_message().await?;

    let interaction_collector = message.await_component_interactions(ctx);

    let duration = Duration::from_secs(60 * 3);

    let result =
        if let Ok(Some(interaction)) = timeout(duration, interaction_collector.next()).await {
            interaction
                .create_response(ctx, CreateInteractionResponse::Acknowledge)
                .await?;

            let data_kind = interaction.data.kind;
            if let ComponentInteractionDataKind::StringSelect { values: uuids } = data_kind {
                let uuids = uuids.into_iter().collect::<Vec<_>>();
                for uuid in uuids {
                    queries::violations::delete(db, &uuid).await?;
                }

                violations -= 1;
                if violations < 0 {
                    violations = 0;
                }

                queries::users::update_violations(db, &user_id, violations).await?;
            }

            info!("@{author_name} removed warning from @{user_name} in {guild_name}");
            Ok(format!("Removed a warning from {user_mention}."))
        } else {
            error!("Failed to remove warning from @{user_name} in {guild_name}");
            Err(format!("Took too long to respond."))
        };

    let reply = match result {
        Ok(message) => components::replies::ok_reply_embed(message, true),
        Err(message) => components::replies::error_reply_embed(message, true),
    };

    ctx.send(reply).await?;

    Ok(())
}
