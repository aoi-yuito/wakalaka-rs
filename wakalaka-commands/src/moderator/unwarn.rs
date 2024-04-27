// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::time::Duration;

use serenity::{
    all::{
        ComponentInteractionDataKind, CreateActionRow, CreateSelectMenu, CreateSelectMenuKind,
        Mentionable, User,
    },
    futures,
};
use uuid::Uuid;

use wakalaka_core::{
    accessors, builders,
    types::{Context, Throwable},
};
use wakalaka_database::queries;

#[poise::command(
    slash_command,
    category = "Moderator",
    required_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "SEND_MESSAGES | MODERATE_MEMBERS",
    guild_only,
    user_cooldown = 5,
    ephemeral
)]
/// Unwarn a user.
pub(super) async fn unwarn(
    ctx: Context<'_>,
    #[description = "User to unwarn."] user: User,
) -> Throwable<()> {
    if crate::is_user_bot_or_system(ctx, &user).await? {
        return Ok(());
    }

    let data = ctx.data();
    let db = &data.db;

    let author = ctx.author();
    let author_id = &author.id;
    let author_name = &author.name;

    let user_id = &user.id;
    let user_name = &user.name;
    let user_mention = user.mention();

    if user_id == author_id {
        let reply =
            builders::replies::build_error_reply_with_embed("Cannot unwarn yourself.", true);

        ctx.send(reply).await?;

        return Ok(());
    }

    let guild = accessors::guilds::fetch_guild(ctx)?;
    let guild_id = &guild.id;
    let guild_name = &guild.name;

    if let Err(_) = queries::users::fetch_user_id_from_db(db, user_id).await {
        let reply = builders::replies::build_error_reply_with_embed(
            format!("Cannot find {user_mention} in database."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let mut user_warns = queries::users::fetch_warnings_from_db(db, user_id).await?;
    if user_warns < 1 {
        let reply = builders::replies::build_error_reply_with_embed(
            format!("Cannot find any warnings for {user_mention} in database."),
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let warns = queries::warnings::gather_all_from_db(db, guild_id, user_id).await?;

    let select_menu_opts = futures::future::join_all(warns.iter().map(
        |(uuid, _, moderator_id, reason, created_at)| {
            builders::select_menu_options::build_warning_select_menu_option(
                ctx,
                uuid,
                moderator_id,
                reason,
                created_at,
            )
        },
    ))
    .await;
    let select_menu_kind = CreateSelectMenuKind::String {
        options: select_menu_opts,
    };
    let select_menu = CreateSelectMenu::new("restricted_guild_select_menu", select_menu_kind)
        .placeholder("@username - Jan 1, 1970)")
        .min_values(1)
        .max_values(1);

    let action_row = CreateActionRow::SelectMenu(select_menu);

    let mut reply = builders::replies::build_reply_with_optional_embed(
        "Which warning would you like to remove?",
        &None,
        true,
    )
    .components(vec![action_row]);

    let message = ctx.send(reply).await?.into_message().await?;

    let expires_in = Duration::from_secs(60 * 3); // 3 minutes

    let collector = message
        .await_component_interactions(ctx)
        .timeout(expires_in);

    let result = match collector.await {
        Some(interact) => {
            let interact_data = &interact.data;
            let interact_data_kind = &interact_data.kind;
            match interact_data_kind {
                ComponentInteractionDataKind::StringSelect { values } => {
                    let uuid = &values
                        .first()
                        .and_then(|value| Uuid::parse_str(value).ok())
                        .ok_or("Failed to parse UUID")?;

                    queries::warnings::remove_warning_from_db(db, uuid).await?;

                    user_warns -= 1;

                    queries::users::update_warnings_in_db(db, user_id, user_warns).await?;

                    interact.delete_response(ctx).await?;

                    tracing::info!("@{author_name} unwarned @{user_name} in {guild_name}");

                    Ok(format!("{user_mention} has been unwarned."))
                }
                _ => {
                    tracing::error!("Unhandled interaction data kind: {interact_data_kind:?}");

                    return Ok(());
                }
            }
        }
        None => Err(format!("Took too long to respond.")),
    };

    reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true).components(vec![]),
        Err(emsg) => builders::replies::build_error_reply_with_embed(emsg, true).components(vec![]),
    };

    ctx.send(reply).await?;

    Ok(())
}
