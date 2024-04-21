// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateInteractionResponse, Interaction};
use tracing::{error, info};
use wakalaka_core::types::{SContext, Throwable};
use wakalaka_utils::accessors;

pub(crate) async fn handle_interaction_create_event(
    ctx: &SContext,
    interact: &Interaction,
) -> Throwable<()> {
    match interact {
        Interaction::Command(interact) => {
            let user = &interact.user;
            let user_name = &user.name;

            let cmd_data = &interact.data;
            let cmd_name = &cmd_data.name;

            let guild_id = &interact.guild_id.expect("Guild ID not found");
            let guild_name = accessors::guilds::fetch_raw_guild_name(ctx, guild_id);

            info!("@{user_name} executed /{cmd_name} in {guild_name}");
        }
        Interaction::Component(interact) => {
            interact
                .create_response(ctx, CreateInteractionResponse::Acknowledge)
                .await?; // Keep, so wouldn't have to call it every interaction awaitance. Ever.
        }
        _ => {
            error!("Unhandled interaction: {interact:?}");

            return Ok(());
        }
    }

    Ok(())
}
