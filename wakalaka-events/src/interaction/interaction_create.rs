// Copyright (c) 2024 Kawaxte
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serenity::all::{CreateInteractionResponse, Interaction};

use wakalaka_core::{
    accessors,
    types::{SContext, Throwable},
};

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

            tracing::info!("@{user_name} executed /{cmd_name} in {guild_name}");
        }
        Interaction::Component(interact) => {
            interact
                .create_response(ctx, CreateInteractionResponse::Acknowledge)
                .await?;
        }
        _ => {
            tracing::error!("Unhandled interaction: {interact:?}");

            return Ok(());
        }
    }

    Ok(())
}
