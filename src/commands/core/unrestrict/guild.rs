// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{str::FromStr, time::Duration};

use serenity::{
    all::{
        ComponentInteractionDataKind, CreateActionRow, CreateSelectMenu, CreateSelectMenuKind,
        GuildId,
    },
    futures,
};
use tracing::error;
use wakalaka_core::types::{Context, Throwable};
use wakalaka_db::queries;
use wakalaka_utils::builders;

#[poise::command(
    slash_command,
    category = "Core",
    required_permissions = "ADMINISTRATOR",
    required_bot_permissions = "SEND_MESSAGES",
    owners_only,
    user_cooldown = 5,
    ephemeral
)]
/// Allow a server to have yours truly in it.
pub(super) async fn guild(ctx: Context<'_>) -> Throwable<()> {
    let data = ctx.data();
    let db = &data.db;

    let restricted_guilds =
        queries::restricted_guilds::gather_all_restricted_guilds_from_db(db).await?;
    if restricted_guilds.is_empty() {
        let reply = builders::replies::build_error_reply_with_embed(
            "Cannot find any servers in database.",
            true,
        );

        ctx.send(reply).await?;

        return Ok(());
    }

    let sel_menu_opts = futures::future::join_all(restricted_guilds.iter().map(
        |(guild_id, reason, created_at)| {
            builders::select_menu_options::build_restricted_guild_select_menu_option(
                guild_id, reason, created_at,
            )
        },
    ))
    .await;
    let sel_menu_kind = CreateSelectMenuKind::String {
        options: sel_menu_opts,
    };
    let sel_menu = CreateSelectMenu::new("restricted_guild_selection", sel_menu_kind)
        .placeholder("123456789123456789 (Jan 1, 1970)")
        .min_values(1)
        .max_values(1);

    let action_row = CreateActionRow::SelectMenu(sel_menu);

    let mut reply = builders::replies::build_reply_with_optional_embed(
        "Which server would you like to unrestrict?",
        &None,
        true,
    )
    .components(vec![action_row]);

    let message = ctx.send(reply).await?.into_message().await?;

    let expires_in = Duration::from_secs(60 * 3); // 3 minutes

    let interact_col = message
        .await_component_interactions(ctx)
        .timeout(expires_in);

    let result = match interact_col.await {
        Some(interact) => {
            let interact_data = &interact.data;
            let interact_data_kind = &interact_data.kind;
            match interact_data_kind {
                ComponentInteractionDataKind::StringSelect { values } => {
                    let guild_id = &values
                        .first()
                        .and_then(|value| GuildId::from_str(value).ok())
                        .ok_or("Failed to parse guild ID")?;

                    queries::restricted_guilds::remove_restricted_guild_from_db(db, guild_id)
                        .await?;

                    interact.delete_response(ctx).await?;

                    Ok(format!(
                        "`{guild_id}` is no longer restricted from having yours truly in it."
                    ))
                }
                _ => {
                    error!("Unhandled interaction data kind: {interact_data_kind:?}");

                    return Ok(());
                }
            }
        }
        None => Err(format!("Took too long to respond.")),
    };

    reply = match result {
        Ok(msg) => builders::replies::build_success_reply_with_embed(msg, true).components(vec![]),
        Err(msg) => builders::replies::build_error_reply_with_embed(msg, true).components(vec![]),
    };

    ctx.send(reply).await?;

    Ok(())
}
